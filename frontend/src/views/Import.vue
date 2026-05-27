<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import {
  NUpload,
  NButton,
  NCard,
  NTag,
  NSpace,
  NAlert,
  NSpin,
  NDivider,
  NInput,
  useMessage,
} from 'naive-ui'
import type { UploadFileInfo } from 'naive-ui'
import api from '../lib/api'
import { useAuth } from '../composables/useAuth'

const router = useRouter()
const message = useMessage()
const { isAuthenticated, setAdminKey } = useAuth()

// 状态
const fileContent = ref<string | null>(null)
const parsedData = ref<ParsedFrontmatter | null>(null)
const isLoading = ref(false)
const isImporting = ref(false)
const error = ref<string | null>(null)
const showAuthDialog = ref(false)
const adminKeyInput = ref('')

const isDragOver = ref(false)

// 类型
interface ParsedFrontmatter {
  title: string
  content: string
  excerpt?: string
  coverImage?: string
  status: string
  tags: string[]
  date?: string
}

/**
 * 简单的 frontmatter 解析器
 * 解析 --- 分隔符之间的类 YAML frontmatter
 */
function parseFrontmatter(markdown: string): ParsedFrontmatter {
  const lines = markdown.split('\n')
  let frontmatterEnd = -1
  let frontmatterStart = -1

  // 查找 frontmatter 边界
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim()
    if (line === '---') {
      if (frontmatterStart === -1) {
        frontmatterStart = i
      } else {
        frontmatterEnd = i
        break
      }
    }
  }

  // 解析 frontmatter（如果找到）
  const metadata: Record<string, string> = {}
  if (frontmatterStart !== -1 && frontmatterEnd !== -1) {
    for (let i = frontmatterStart + 1; i < frontmatterEnd; i++) {
      const line = lines[i].trim()
      if (line && line.includes(':')) {
        const colonIndex = line.indexOf(':')
        const key = line.substring(0, colonIndex).trim()
        const value = line.substring(colonIndex + 1).trim()
        // 如果存在则移除引号
        metadata[key] = value.replace(/^["']|["']$/g, '')
      }
    }
  }

  // 提取内容（frontmatter 之后的所有内容）
  const contentStart = frontmatterEnd !== -1 ? frontmatterEnd + 1 : 0
  const content = lines.slice(contentStart).join('\n').trim()

  // 解析标签（逗号分隔或数组格式）
  let tags: string[] = []
  if (metadata.tags) {
    // 处理 [tag1, tag2] 格式或逗号分隔
    const tagsStr = metadata.tags.replace(/[\[\]]/g, '')
    tags = tagsStr.split(',').map(t => t.trim()).filter(Boolean)
  }

  return {
    title: metadata.title || 'Untitled',
    content,
    excerpt: metadata.excerpt || metadata.description,
    coverImage: metadata.cover_image || metadata.coverImage || metadata.image,
    status: metadata.status || 'draft',
    tags,
    date: metadata.date,
  }
}

/**
 * 处理文件上传 - 读取并解析 markdown 文件
 */
function handleFileUpload(options: { file: UploadFileInfo }) {
  const file = options.file.file
  if (!file) return

  // 验证文件类型
  if (!file.name.endsWith('.md')) {
    error.value = 'Unsupported file format. Please upload a .md file.'
    message.error('Unsupported file format')
    return
  }

  error.value = null
  isLoading.value = true

  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      fileContent.value = content
      parsedData.value = parseFrontmatter(content)
      isLoading.value = false
    } catch (err) {
      error.value = 'Failed to parse the markdown file.'
      message.error('Parse error')
      isLoading.value = false
    }
  }
  reader.onerror = () => {
    error.value = 'Failed to read the file.'
    message.error('Read error')
    isLoading.value = false
  }
  reader.readAsText(file)
}

/**
 * 处理拖拽事件
 */
function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = true
}

function handleDragLeave() {
  isDragOver.value = false
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = false
  
  const files = e.dataTransfer?.files
  if (!files || files.length === 0) return
  
  const file = files[0]
  if (!file.name.endsWith('.md')) {
    error.value = 'Unsupported file format. Please upload a .md file.'
    message.error('Unsupported file format')
    return
  }
  
  error.value = null
  isLoading.value = true
  
  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      fileContent.value = content
      parsedData.value = parseFrontmatter(content)
      isLoading.value = false
    } catch (err) {
      error.value = 'Failed to parse the markdown file.'
      message.error('Parse error')
      isLoading.value = false
    }
  }
  reader.onerror = () => {
    error.value = 'Failed to read the file.'
    message.error('Read error')
    isLoading.value = false
  }
  reader.readAsText(file)
}

/**
 * 确认导入 - 发送到 API
 */
async function confirmImport() {
  if (!parsedData.value) return

  // 导入前检查认证
  if (!isAuthenticated.value) {
    showAuthDialog.value = true
    return
  }

  isImporting.value = true
  error.value = null

  try {
    const response = await api.post('/api/v1/posts', {
      title: parsedData.value.title,
      content: parsedData.value.content,
      excerpt: parsedData.value.excerpt,
      cover_image: parsedData.value.coverImage,
      status: parsedData.value.status,
    })

    message.success('导入成功！')
    router.push(`/posts/${response.data.id}`)
  } catch (err: any) {
    const status = err.response?.status
    if (status === 401) {
      message.error('认证失败，请检查管理密钥')
      showAuthDialog.value = true
    } else {
      const errorMsg = err.response?.data?.message || err.response?.data?.error || '导入失败'
      error.value = errorMsg
      message.error(errorMsg)
    }
  } finally {
    isImporting.value = false
  }
}

/**
 * 处理认证提交
 */
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

/**
 * 重置导入状态
 */
function resetImport() {
  fileContent.value = null
  parsedData.value = null
  error.value = null
}

// 计算属性
const hasPreview = computed(() => parsedData.value !== null)
const formattedDate = computed(() => {
  if (!parsedData.value?.date) return null
  try {
    return new Date(parsedData.value.date).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  } catch {
    return parsedData.value.date
  }
})
</script>

<template>
  <div class="import-page">
    <!-- Header -->
    <header class="import-header">
      <h2 class="import-title">导入文章</h2>
      <p class="import-subtitle">
        从 Obsidian 或其他 Markdown 编辑器导入文章
      </p>
    </header>

    <!-- Error Alert -->
    <n-alert
      v-if="error"
      type="error"
      :bordered="false"
      closable
      @close="error = null"
      class="import-alert"
    >
      {{ error }}
    </n-alert>

    <!-- Upload Section -->
    <n-card v-if="!hasPreview" class="upload-card" :bordered="false">
      <div class="upload-area">
        <n-upload
          :max="1"
          accept=".md"
          :default-upload="false"
          @change="handleFileUpload"
          :disabled="isLoading"
        >
          <div 
            class="upload-trigger"
            :class="{ 'upload-trigger--drag-over': isDragOver }"
            @dragover.prevent="handleDragOver"
            @dragleave="handleDragLeave"
            @drop.prevent="handleDrop"
          >
            <div class="upload-icon">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17 8 12 3 7 8" />
                <line x1="12" y1="3" x2="12" y2="15" />
              </svg>
            </div>
            <div class="upload-text">
              <span class="upload-text__primary">点击或拖拽上传 Markdown 文件</span>
              <span class="upload-text__secondary">仅支持 .md 格式</span>
            </div>
          </div>
        </n-upload>

        <div v-if="isLoading" class="upload-loading">
          <n-spin size="medium" />
          <span>解析中...</span>
        </div>
      </div>
    </n-card>

    <!-- Preview Section -->
    <div v-if="hasPreview" class="preview-section">
      <!-- Preview Card -->
      <n-card class="preview-card" :bordered="false">
        <template #header>
          <div class="preview-header">
            <span class="preview-label">预览</span>
            <n-button text @click="resetImport" class="reset-btn">
              重新选择
            </n-button>
          </div>
        </template>

        <!-- Title -->
        <h3 class="preview-title">{{ parsedData?.title }}</h3>

        <!-- Meta Info -->
        <div class="preview-meta">
          <n-space v-if="parsedData?.tags.length" :size="8">
            <n-tag
              v-for="tag in parsedData?.tags"
              :key="tag"
              size="small"
              :bordered="false"
              type="info"
            >
              {{ tag }}
            </n-tag>
          </n-space>
          <span v-if="formattedDate" class="preview-date">
            {{ formattedDate }}
          </span>
          <n-tag
            :type="parsedData?.status === 'published' ? 'success' : 'warning'"
            size="small"
            :bordered="false"
          >
            {{ parsedData?.status === 'published' ? '已发布' : '草稿' }}
          </n-tag>
        </div>

        <!-- Excerpt -->
        <p v-if="parsedData?.excerpt" class="preview-excerpt">
          {{ parsedData.excerpt }}
        </p>

        <n-divider />

        <!-- Content Preview -->
        <div class="preview-content">
          <div class="content-label">内容预览</div>
          <div class="content-body">
            {{ parsedData?.content.substring(0, 500) }}{{ parsedData && parsedData.content.length > 500 ? '...' : '' }}
          </div>
        </div>
      </n-card>

      <!-- Action Buttons -->
      <div class="import-actions">
        <n-button
          size="large"
          @click="resetImport"
          :disabled="isImporting"
        >
          取消
        </n-button>
        <n-button
          type="primary"
          size="large"
          :loading="isImporting"
          @click="confirmImport"
        >
          {{ isImporting ? '导入中...' : '确认导入' }}
        </n-button>
      </div>
    </div>

    <!-- Auth Dialog -->
    <div v-if="showAuthDialog" class="auth-overlay" @click.self="showAuthDialog = false">
      <div class="auth-dialog">
        <h3 class="auth-dialog__title">管理员认证</h3>
        <p class="auth-dialog__desc">
          请输入管理密钥以导入文章
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
/* 导入页面布局 */
.import-page {
  max-width: 720px;
  margin: 0 auto;
  padding: var(--space-8) var(--space-4);
}

/* 头部 */
.import-header {
  margin-bottom: var(--space-8);
  text-align: center;
}

.import-title {
  font-family: var(--font-display);
  font-size: var(--text-3xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.import-subtitle {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  font-weight: 400;
}

/* 警告 */
.import-alert {
  margin-bottom: var(--space-6);
  border-radius: var(--radius-md);
}

/* 上传卡片 */
.upload-card {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
}

.upload-area {
  padding: var(--space-8);
}

.upload-trigger {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-12) var(--space-8);
  border: 2px dashed var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-base);
  background: var(--color-bg);
}

.upload-trigger:hover {
  border-color: var(--color-accent);
  background: var(--color-accent-light);
}

.upload-trigger--drag-over {
  border-color: var(--color-accent);
  background: var(--color-accent-light);
  transform: scale(1.01);
}

.upload-icon {
  color: var(--color-text-tertiary);
  transition: color var(--transition-base);
}

.upload-trigger:hover .upload-icon {
  color: var(--color-accent);
}

.upload-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-1);
}

.upload-text__primary {
  font-size: var(--text-lg);
  font-weight: 500;
  color: var(--color-text-primary);
}

.upload-text__secondary {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

.upload-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-3);
  padding: var(--space-6);
  color: var(--color-text-secondary);
}

/* 预览区域 */
.preview-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.preview-card {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.preview-label {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.reset-btn {
  font-size: var(--text-sm);
  color: var(--color-accent);
}

.reset-btn:hover {
  color: var(--color-accent-hover);
}

/* 预览内容 */
.preview-title {
  font-family: var(--font-display);
  font-size: var(--text-2xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-4);
  line-height: 1.3;
}

.preview-meta {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.preview-date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

.preview-excerpt {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  line-height: 1.6;
  font-style: italic;
  margin-bottom: var(--space-4);
  padding-left: var(--space-4);
  border-left: 3px solid var(--color-border);
}

/* 内容预览 */
.preview-content {
  margin-top: var(--space-4);
}

.content-label {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--space-3);
}

.content-body {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  line-height: 1.7;
  white-space: pre-wrap;
  font-family: var(--font-body);
  max-height: 300px;
  overflow-y: auto;
  padding: var(--space-4);
  background: var(--color-bg);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border-light);
}

/* 操作按钮 */
.import-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding-top: var(--space-4);
}

/* 响应式 */
@media (max-width: 768px) {
  .import-page {
    padding: var(--space-4) var(--space-3);
  }

  .import-title {
    font-size: var(--text-2xl);
  }

  .upload-trigger {
    padding: var(--space-8) var(--space-4);
  }

  .upload-text__primary {
    font-size: var(--text-base);
  }

  .preview-title {
    font-size: var(--text-xl);
  }

  .import-actions {
    flex-direction: column;
  }

  .import-actions .n-button {
    width: 100%;
  }
}

/* 上传组件覆盖 */
:deep(.n-upload) {
  width: 100%;
}

:deep(.n-upload-trigger) {
  width: 100%;
}

:deep(.n-upload-file-list) {
  display: none;
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
