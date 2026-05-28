<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NButton, NInput, NSelect, useMessage } from 'naive-ui'
import { useDebounceFn } from '@vueuse/core'
import api from '../lib/api'
import { useAuth } from '../composables/useAuth'
import { useTags } from '../composables/useTags'

const router = useRouter()
const route = useRoute()
const message = useMessage()
const { isAuthenticated, setAdminKey } = useAuth()
const { tags, fetchTags, assignTagsToPost } = useTags()

// --- 状态 ---
const title = ref('')
const content = ref('')
const titleInputRef = ref<HTMLInputElement | null>(null)
const contentRef = ref<HTMLTextAreaElement | null>(null)
const editorRef = ref<HTMLDivElement | null>(null)
const isSaving = ref(false)
const lastSavedAt = ref<Date | null>(null)
const isDragging = ref(false)
const isPublishing = ref(false)
const isEditing = ref(false) // true when editing an already-published post
const editPostId = ref<string | null>(null)
const selectedTagIds = ref<number[]>([])

// --- 认证弹窗 ---
const showAuthDialog = ref(false)
const adminKeyInput = ref('')

// --- localStorage 草稿键 ---
const DRAFT_KEY = 'zyblog_draft'

// --- 防抖自动保存 ---
const debouncedSave = useDebounceFn(() => {
  saveDraft()
}, 1500)

function saveDraft() {
  if (!title.value && !content.value) return
  if (isEditing.value) return

  isSaving.value = true
  const draft = {
    title: title.value,
    content: content.value,
    savedAt: new Date().toISOString(),
  }

  localStorage.setItem(DRAFT_KEY, JSON.stringify(draft))

  setTimeout(() => {
    isSaving.value = false
    lastSavedAt.value = new Date()
  }, 300)
}

function loadDraft() {
  const routeId = route.params.id as string | undefined

  if (routeId) {
    isEditing.value = true
    editPostId.value = routeId
    fetchPostForEdit(routeId)
    return
  }

  // 加载常规草稿
  const saved = localStorage.getItem(DRAFT_KEY)
  if (saved) {
    try {
      const draft = JSON.parse(saved)
      title.value = draft.title || ''
      content.value = draft.content || ''
      if (draft.savedAt) {
        lastSavedAt.value = new Date(draft.savedAt)
      }
    } catch {
      // 数据损坏，忽略
    }
  }
}

async function fetchPostForEdit(id: string) {
  try {
    const { data } = await api.get(`/api/v1/posts/${id}`)
    title.value = data.title || ''
    content.value = data.content || ''
  } catch {
    message.error('无法加载文章')
    router.push('/drafts')
  }
}

function clearDraft() {
  localStorage.removeItem(DRAFT_KEY)
  title.value = ''
  content.value = ''
  lastSavedAt.value = null
}

// --- 监听变化 ---
watch([title, content], () => {
  debouncedSave()
})

// --- 保存状态显示 ---
const savedStatusText = computed(() => {
  if (isSaving.value) return '保存中...'
  if (!lastSavedAt.value) return ''
  const now = new Date()
  const diff = now.getTime() - lastSavedAt.value.getTime()
  if (diff < 5000) return '已保存'
  if (diff < 60000) return `${Math.floor(diff / 1000)}秒前保存`
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前保存`
  return '已保存草稿'
})

// --- 标签选项 ---
const tagOptions = computed(() =>
  tags.value.map((tag) => ({ label: tag.name, value: tag.id }))
)

// --- 格式化 ---
function wrapSelection(prefix: string, suffix: string = '') {
  const textarea = contentRef.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selected = content.value.substring(start, end)
  const replacement = `${prefix}${selected}${suffix}`

  content.value =
    content.value.substring(0, start) + replacement + content.value.substring(end)

  nextTick(() => {
    textarea.focus()
    const newCursorPos = start + prefix.length + selected.length + suffix.length
    textarea.setSelectionRange(newCursorPos, newCursorPos)
  })
}

function toggleBold() {
  wrapSelection('**', '**')
}

function insertList() {
  const textarea = contentRef.value
  if (!textarea) return

  const start = textarea.selectionStart
  const lineStart = content.value.lastIndexOf('\n', start - 1) + 1
  const lineEnd = content.value.indexOf('\n', start)
  const actualEnd = lineEnd === -1 ? content.value.length : lineEnd
  const line = content.value.substring(lineStart, actualEnd)

  if (line.startsWith('- ')) {
    // 移除列表标记
    content.value =
      content.value.substring(0, lineStart) +
      line.substring(2) +
      content.value.substring(actualEnd)
  } else {
    // 添加列表标记
    content.value =
      content.value.substring(0, lineStart) +
      '- ' + line +
      content.value.substring(actualEnd)
  }

  nextTick(() => {
    textarea.focus()
  })
}

// --- 视频处理 ---
function insertVideoMarkdown(url: string) {
  const textarea = contentRef.value
  if (!textarea) return

  const start = textarea.selectionStart
  const markdown = `!video[${url}]`

  content.value =
    content.value.substring(0, start) + markdown + content.value.substring(start)

  nextTick(() => {
    textarea.focus()
    const newPos = start + markdown.length
    textarea.setSelectionRange(newPos, newPos)
  })
}

function handleVideoUpload() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'video/mp4,video/webm,video/ogg'
  input.onchange = async (e: Event) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (!file) return
    await processVideoFile(file)
  }
  input.click()
}

async function processVideoFile(file: File) {
  const formData = new FormData()
  formData.append('file', file)

  try {
    const { data } = await api.post('/api/v1/videos', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    })
    insertVideoMarkdown(data.url)
    message.success('视频已上传')
  } catch {
    message.error('视频上传失败')
  }
}

// --- 图片处理 ---
function insertImageMarkdown(url: string, alt: string = 'image') {
  const textarea = contentRef.value
  if (!textarea) return

  const start = textarea.selectionStart
  const markdown = `![${alt}](${url})`

  content.value =
    content.value.substring(0, start) + markdown + content.value.substring(start)

  nextTick(() => {
    textarea.focus()
    const newPos = start + markdown.length
    textarea.setSelectionRange(newPos, newPos)
  })
}

function handleImageUpload() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'image/*'
  input.onchange = async (e: Event) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (!file) return
    await processImageFile(file)
  }
  input.click()
}

async function processImageFile(file: File) {
  const formData = new FormData()
  formData.append('file', file)

  try {
    const { data } = await api.post('/api/v1/images', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    })
    insertImageMarkdown(data.url, file.name)
    message.success('图片已上传')
  } catch {
    message.error('图片上传失败')
  }
}

// 图片粘贴处理器
function handlePaste(e: ClipboardEvent) {
  const items = e.clipboardData?.items
  if (!items) return

  for (const item of items) {
    if (item.type.startsWith('image/')) {
      e.preventDefault()
      const file = item.getAsFile()
      if (file) {
        processImageFile(file)
      }
      return
    }
  }
}

// 拖放
function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragging.value = true
}

function handleDragLeave() {
  isDragging.value = false
}

async function handleDrop(e: DragEvent) {
  e.preventDefault()
  isDragging.value = false

  const files = e.dataTransfer?.files
  if (!files) return

  for (const file of files) {
    if (file.type.startsWith('image/')) {
      await processImageFile(file)
    } else if (file.type.startsWith('video/')) {
      await processVideoFile(file)
    }
  }
}

// --- 键盘快捷键 ---
function handleKeydown(e: KeyboardEvent) {
  // Ctrl/Cmd + B 加粗
  if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
    e.preventDefault()
    toggleBold()
  }
  // Ctrl/Cmd + S 保存
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    saveDraft()
  }
  // Ctrl/Cmd + Enter 发布
  if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
    e.preventDefault()
    handlePublish()
  }
}

// --- 认证弹窗处理器 ---
function handleAuthSubmit() {
  if (!adminKeyInput.value.trim()) {
    message.warning('请输入管理密钥')
    return
  }
  setAdminKey(adminKeyInput.value.trim())
  showAuthDialog.value = false
  adminKeyInput.value = ''
  message.success('认证成功')
}

// --- 发布 ---
async function handlePublish() {
  if (!title.value.trim()) {
    message.warning('请输入文章标题')
    return
  }
  if (!content.value.trim()) {
    message.warning('请输入文章内容')
    return
  }

  // 发布前检查认证
  if (!isAuthenticated.value) {
    showAuthDialog.value = true
    return
  }

  isPublishing.value = true

  try {
    let data: { id: number }

    if (isEditing.value && editPostId.value) {
      const response = await api.put(`/api/v1/posts/${editPostId.value}`, {
        title: title.value.trim(),
        content: content.value.trim(),
        status: 'published',
      })
      data = response.data
    } else {
      const response = await api.post('/api/v1/posts', {
        title: title.value.trim(),
        content: content.value.trim(),
        status: 'published',
      })
      data = response.data
    }

    // Assign tags to the newly created/updated post
    if (selectedTagIds.value.length > 0) {
      await assignTagsToPost(data.id, selectedTagIds.value)
    }

    // 发布成功后清除草稿
    clearDraft()

    message.success(isEditing.value ? '更新成功！' : '发布成功！')

    // 导航到文章详情
    router.push(`/posts/${data.id}`)
  } catch (err: any) {
    const status = err.response?.status
    if (status === 401) {
      message.error('认证失败，请检查管理密钥')
      showAuthDialog.value = true
    } else {
      message.error('发布失败，请重试')
    }
  } finally {
    isPublishing.value = false
  }
}

// --- 生命周期 ---
onMounted(() => {
  loadDraft()
  fetchTags()
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="publish-page">
    <!-- Page Header -->
    <header class="publish-header">
      <div class="publish-header__left">
        <h1 class="publish-header__title">
          {{ isEditing ? '编辑文章' : '写点什么' }}
        </h1>
        <span v-if="savedStatusText" class="publish-header__status" :class="{ 'is-saving': isSaving }">
          {{ savedStatusText }}
        </span>
      </div>
      <div class="publish-header__right">
        <NButton
          quaternary
          size="small"
          @click="clearDraft"
          :disabled="!title && !content"
        >
          清空
        </NButton>
        <NButton
          type="primary"
          :loading="isPublishing"
          @click="handlePublish"
          :disabled="!title.trim() || !content.trim()"
        >
          {{ isPublishing ? (isEditing ? '更新中...' : '发布中...') : (isEditing ? '更新' : '发布') }}
        </NButton>
      </div>
    </header>

    <!-- Editor Area -->
    <div
      ref="editorRef"
      class="publish-editor"
      :class="{ 'is-dragging': isDragging }"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
    >
      <!-- Title Input -->
      <div class="publish-title-wrapper">
        <input
          ref="titleInputRef"
          v-model="title"
          type="text"
          class="publish-title"
          placeholder="无题"
          autocomplete="off"
          spellcheck="false"
        />
        <div class="publish-title__border"></div>
      </div>

      <!-- Formatting Toolbar -->
      <div class="publish-toolbar">
        <button
          class="toolbar-btn"
          title="加粗 (Ctrl+B)"
          @click="toggleBold"
        >
          <span class="toolbar-btn__icon">B</span>
        </button>
        <button
          class="toolbar-btn"
          title="列表"
          @click="insertList"
        >
          <span class="toolbar-btn__icon">☰</span>
        </button>
        <div class="toolbar-divider"></div>
        <button
          class="toolbar-btn"
          title="插入图片"
          @click="handleImageUpload"
        >
          <span class="toolbar-btn__icon">⊕</span>
        </button>
        <button
          class="toolbar-btn"
          title="插入视频"
          @click="handleVideoUpload"
        >
          <span class="toolbar-btn__icon">▶</span>
        </button>
      </div>

      <!-- Tag Selector -->
      <div class="publish-tags">
        <n-select
          v-model:value="selectedTagIds"
          :options="tagOptions"
          multiple
          filterable
          placeholder="选择标签..."
          :max-tag-count="5"
          size="small"
        />
      </div>

      <!-- Content Textarea -->
      <div class="publish-content-wrapper">
        <textarea
          ref="contentRef"
          v-model="content"
          class="publish-content"
          placeholder="开始写作...

支持 Markdown 语法：
**加粗** — 列表项以 - 开头
插入图片/视频：点击上方工具栏或直接粘贴/拖拽"
          @paste="handlePaste"
        ></textarea>
      </div>

      <!-- Drag overlay -->
      <div v-if="isDragging" class="publish-drag-overlay">
        <div class="publish-drag-overlay__content">
          <span class="publish-drag-overlay__icon">⊕</span>
          <span>松开鼠标上传图片或视频</span>
        </div>
      </div>
    </div>

    <!-- Footer Hints -->
    <footer class="publish-footer">
      <span class="publish-footer__hint">
        <kbd>Ctrl</kbd>+<kbd>B</kbd> 加粗 · <kbd>Ctrl</kbd>+<kbd>S</kbd> 保存 · <kbd>Ctrl</kbd>+<kbd>Enter</kbd> 发布
      </span>
      <span class="publish-footer__hint">
        支持粘贴和拖拽图片、视频
      </span>
    </footer>

    <!-- Auth Dialog -->
    <div v-if="showAuthDialog" class="auth-overlay" @click.self="showAuthDialog = false">
      <div class="auth-dialog">
        <h3 class="auth-dialog__title">管理员认证</h3>
        <p class="auth-dialog__desc">
          {{ isEditing ? '请输入管理密钥以更新文章' : '请输入管理密钥以发布文章' }}
        </p>
        <div class="auth-dialog__field">
          <label class="auth-dialog__label" for="admin-key">管理密钥</label>
          <n-input
            id="admin-key"
            v-model:value="adminKeyInput"
            type="password"
            placeholder="请输入 ADMIN_KEY"
            @keyup.enter="handleAuthSubmit"
          />
        </div>
        <div class="auth-dialog__actions">
          <n-button @click="showAuthDialog = false">取消</n-button>
          <n-button type="primary" @click="handleAuthSubmit">确认</n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* --- 页面布局 --- */
.publish-page {
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - var(--space-8) * 2);
  gap: var(--space-4);
}

/* --- 头部 --- */
.publish-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
}

.publish-header__left {
  display: flex;
  align-items: baseline;
  gap: var(--space-4);
}

.publish-header__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
}

.publish-header__status {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  transition: color var(--transition-fast);
}

.publish-header__status.is-saving {
  color: var(--color-accent);
}

.publish-header__right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

/* --- 编辑器容器 --- */
.publish-editor {
  position: relative;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8) var(--space-10);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--color-border-light);
  transition: box-shadow var(--transition-base), border-color var(--transition-base);
}

.publish-editor:focus-within {
  box-shadow: var(--shadow-md);
  border-color: var(--color-border);
}

.publish-editor.is-dragging {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px var(--color-accent-light);
}

/* --- 标题输入 --- */
.publish-title-wrapper {
  position: relative;
}

.publish-title {
  width: 100%;
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  line-height: 1.2;
  color: var(--color-text-primary);
  border: none;
  outline: none;
  background: transparent;
  padding: var(--space-2) 0;
  letter-spacing: -0.03em;
  caret-color: var(--color-accent);
}

.publish-title::placeholder {
  color: var(--color-text-tertiary);
  opacity: 0.6;
}

.publish-title__border {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--color-border-light);
  transition: background var(--transition-base);
}

.publish-title:focus + .publish-title__border {
  background: var(--color-accent);
}

/* --- 工具栏 --- */
.publish-toolbar {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-2) 0;
  opacity: 0.7;
  transition: opacity var(--transition-fast);
}

.publish-editor:focus-within .publish-toolbar {
  opacity: 1;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: background var(--transition-fast), color var(--transition-fast);
  font-family: var(--font-body);
}

.toolbar-btn:hover {
  background: var(--color-bg-sunken);
  color: var(--color-text-primary);
}

.toolbar-btn:active {
  background: var(--color-border-light);
}

.toolbar-btn__icon {
  font-size: var(--text-sm);
  font-weight: 600;
  line-height: 1;
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--color-border-light);
  margin: 0 var(--space-1);
}

/* --- 标签选择器 --- */
.publish-tags {
  padding: var(--space-2) 0;
}

/* --- 内容文本区域 --- */
.publish-content-wrapper {
  flex: 1;
  display: flex;
}

.publish-content {
  width: 100%;
  min-height: 400px;
  flex: 1;
  font-family: var(--font-body);
  font-size: var(--text-lg);
  line-height: 1.8;
  color: var(--color-text-primary);
  border: none;
  outline: none;
  background: transparent;
  resize: none;
  padding: var(--space-2) 0;
  caret-color: var(--color-accent);
}

.publish-content::placeholder {
  color: var(--color-text-tertiary);
  opacity: 0.5;
  font-size: var(--text-base);
  line-height: 2;
}

/* --- 拖拽遮罩 --- */
.publish-drag-overlay {
  position: absolute;
  inset: 0;
  background: rgba(196, 93, 62, 0.06);
  border: 2px dashed var(--color-accent);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  pointer-events: none;
}

.publish-drag-overlay__content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  color: var(--color-accent);
  font-size: var(--text-lg);
  font-weight: 500;
}

.publish-drag-overlay__icon {
  font-size: 48px;
  line-height: 1;
  opacity: 0.6;
}

/* --- 底部 --- */
.publish-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: var(--space-3);
  border-top: 1px solid var(--color-border-light);
}

.publish-footer__hint {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

kbd {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 1px 5px;
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--color-text-secondary);
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  box-shadow: 0 1px 0 var(--color-border);
  line-height: 1.4;
}

/* --- 移动端响应式 --- */
@media (max-width: 767px) {
  .publish-editor {
    padding: var(--space-4) var(--space-4);
    border-radius: var(--radius-md);
  }

  .publish-title {
    font-size: var(--text-2xl);
  }

  .publish-content {
    font-size: var(--text-base);
    min-height: 300px;
  }

  .publish-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }

  .publish-header__right {
    width: 100%;
    justify-content: flex-end;
  }

  .publish-footer {
    flex-direction: column;
    gap: var(--space-2);
    align-items: flex-start;
  }
}

@media (min-width: 768px) and (max-width: 1023px) {
  .publish-editor {
    padding: var(--space-6) var(--space-8);
  }

  .publish-title {
    font-size: var(--text-3xl);
  }
}

/* --- 认证弹窗 --- */
.auth-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.auth-dialog {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 400px;
  box-shadow: var(--shadow-lg);
}

.auth-dialog__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.auth-dialog__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-6);
}

.auth-dialog__field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-bottom: var(--space-6);
}

.auth-dialog__label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.auth-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}
</style>
