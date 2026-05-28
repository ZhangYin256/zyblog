<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { NSpin, NButton, NInput, NSelect, useMessage } from 'naive-ui'
import { usePullRequests, type Fragment, type PullRequest, type Comment } from '../composables/usePullRequests'
import { useAuth } from '../composables/useAuth'

const props = defineProps<{
  postId: number
  postContent?: string
  postBodyEl?: HTMLElement | null
}>()

const emit = defineEmits<{
  'pr-created': []
  'pr-applied': []
}>()

const message = useMessage()
const { isAuthenticated, user } = useAuth()

// ── Composable ──────────────────────────────────────────

const postIdRef = computed(() => props.postId)
const {
  pulls, loading, error,
  fetchPulls, createPull, applyPull,
  addComment, fetchComments,
  formatDate, getStatusColor, getStatusLabel,
} = usePullRequests(postIdRef)

// ── Status Filter ───────────────────────────────────────

const statusFilter = ref('all')
const statusOptions = [
  { label: '全部', value: 'all' },
  { label: '开放', value: 'open' },
  { label: '已关闭', value: 'closed' },
  { label: '已合并', value: 'merged' },
]

const filteredPulls = computed(() =>
  statusFilter.value === 'all'
    ? pulls.value
    : pulls.value.filter(p => p.status === statusFilter.value),
)

// ── Text Selection ──────────────────────────────────────

const selectedText = ref('')
const selectionPositions = ref<{
  start_line: number
  start_col: number
  end_line: number
  end_col: number
} | null>(null)

function handleSelectionChange() {
  const container = props.postBodyEl
  if (!container) return

  const sel = window.getSelection()
  if (!sel || sel.isCollapsed || !sel.rangeCount) {
    selectedText.value = ''
    selectionPositions.value = null
    return
  }

  const range = sel.getRangeAt(0)
  if (
    !container.contains(range.startContainer) ||
    !container.contains(range.endContainer)
  ) {
    selectedText.value = ''
    selectionPositions.value = null
    return
  }

  const text = sel.toString().trim()
  if (!text) {
    selectedText.value = ''
    selectionPositions.value = null
    return
  }

  // Prefer calculating positions from source content (postContent) because
  // the backend validates against source text, not rendered DOM text.
  // Rendered text differs from source for markdown (e.g. **bold** → bold).
  if (props.postContent) {
    const sourceIdx = props.postContent.indexOf(text)
    if (sourceIdx !== -1) {
      const beforeStart = props.postContent.substring(0, sourceIdx)
      const beforeEnd = props.postContent.substring(0, sourceIdx + text.length)
      const startLines = beforeStart.split('\n')
      const endLines = beforeEnd.split('\n')

      selectedText.value = text
      selectionPositions.value = {
        start_line: startLines.length,
        start_col: startLines[startLines.length - 1].length,
        end_line: endLines.length,
        end_col: endLines[endLines.length - 1].length,
      }
      return
    }
  }

  // Fallback: calculate from rendered text content
  const preRange = document.createRange()
  preRange.selectNodeContents(container)
  preRange.setEnd(range.startContainer, range.startOffset)
  const startOffset = preRange.toString().length
  const endOffset = startOffset + text.length

  const fullText = container.textContent || ''
  const startLines = fullText.substring(0, startOffset).split('\n')
  const endLines = fullText.substring(0, endOffset).split('\n')

  selectedText.value = text
  selectionPositions.value = {
    start_line: startLines.length,
    start_col: startLines[startLines.length - 1].length,
    end_line: endLines.length,
    end_col: endLines[endLines.length - 1].length,
  }
}

onMounted(() => document.addEventListener('selectionchange', handleSelectionChange))
onUnmounted(() => document.removeEventListener('selectionchange', handleSelectionChange))

// ── Create PR Modal ─────────────────────────────────────

const showCreateModal = ref(false)
const createReplacement = ref('')
const createDescription = ref('')
const createMessage = ref('')
const createEmail = ref('')
const creating = ref(false)

function openCreateModal() {
  if (!selectedText.value || !selectionPositions.value) return
  createReplacement.value = selectedText.value
  createDescription.value = ''
  createMessage.value = ''
  createEmail.value = ''
  showCreateModal.value = true
}

function cancelCreate() {
  showCreateModal.value = false
}

async function handleCreate() {
  if (!selectionPositions.value) return
  if (!createReplacement.value.trim()) {
    message.warning('请输入替换内容')
    return
  }

  const email = isAuthenticated.value && user.value ? user.value.email : createEmail.value.trim()
  if (!email) {
    message.warning('请输入邮箱地址')
    return
  }

  const fragment: Fragment = {
    ...selectionPositions.value,
    replacement: createReplacement.value.trim(),
    description: createDescription.value.trim() || undefined,
  }

  creating.value = true
  try {
    await createPull([fragment], createMessage.value.trim(), email)
    message.success('PR 创建成功！')
    showCreateModal.value = false
    selectedText.value = ''
    selectionPositions.value = null
    window.getSelection()?.removeAllRanges()
    emit('pr-created')
  } catch (err: unknown) {
    let msg = '创建失败，请重试'
    if (err && typeof err === 'object' && 'response' in err) {
      const resp = (err as { response?: { data?: { error?: string } } }).response
      if (resp?.data?.error) msg = resp.data.error
    } else if (err instanceof Error) {
      msg = err.message
    }
    message.error(msg)
  } finally {
    creating.value = false
  }
}

// ── PR Detail ───────────────────────────────────────────

const selectedPull = ref<PullRequest | null>(null)
const pullComments = ref<Comment[]>([])
const detailLoading = ref(false)
const commentForms = ref<Record<number, { email: string; content: string; submitting: boolean }>>({})

function getCommentForm(index: number) {
  if (!commentForms.value[index]) {
    commentForms.value[index] = { email: '', content: '', submitting: false }
  }
  return commentForms.value[index]
}

function getFragmentComments(index: number): Comment[] {
  return pullComments.value.filter(c => c.fragment_index === index)
}

function getOriginalText(frag: Fragment): string {
  if (!props.postContent) return ''
  const lines = props.postContent.split('\n')
  if (frag.start_line === frag.end_line) {
    const line = lines[frag.start_line - 1] || ''
    return line.substring(frag.start_col, frag.end_col)
  }
  const result: string[] = []
  for (let i = frag.start_line - 1; i < frag.end_line && i < lines.length; i++) {
    const line = lines[i]
    if (i === frag.start_line - 1) result.push(line.substring(frag.start_col))
    else if (i === frag.end_line - 1) result.push(line.substring(0, frag.end_col))
    else result.push(line)
  }
  return result.join('\n')
}

async function openDetail(pull: PullRequest) {
  selectedPull.value = pull
  detailLoading.value = true
  try {
    pullComments.value = await fetchComments(pull.id)
  } catch {
    pullComments.value = []
  } finally {
    detailLoading.value = false
  }
  commentForms.value = {}
  pull.fragments.forEach((_, i) => {
    commentForms.value[i] = { email: '', content: '', submitting: false }
  })
}

function closeDetail() {
  selectedPull.value = null
  pullComments.value = []
  commentForms.value = {}
  clearHighlights()
}

// ── Fragment Highlighting ───────────────────────────────

function highlightFragments(fragments: Fragment[]) {
  const container = props.postBodyEl
  if (!container) return

  for (const frag of fragments) {
    const searchText = frag.replacement
    if (!searchText) continue

    const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT)
    const textNodes: Text[] = []
    let node: Node | null
    while ((node = walker.nextNode())) textNodes.push(node as Text)

    for (const textNode of textNodes) {
      const content = textNode.textContent || ''
      const idx = content.indexOf(searchText)
      if (idx === -1) continue

      textNode.splitText(idx + searchText.length)
      const match = textNode.splitText(idx)

      const mark = document.createElement('mark')
      mark.className = 'pr-fragment-highlight'
      match.parentNode?.insertBefore(mark, match)
      mark.appendChild(match)

      break
    }
  }
}

function clearHighlights() {
  const container = props.postBodyEl
  if (!container) return
  container.querySelectorAll('.pr-fragment-highlight').forEach(mark => {
    const parent = mark.parentNode
    if (!parent) return
    while (mark.firstChild) parent.insertBefore(mark.firstChild, mark)
    parent.removeChild(mark)
    parent.normalize()
  })
}

watch(selectedPull, (pull) => {
  clearHighlights()
  if (pull) highlightFragments(pull.fragments)
})

onUnmounted(clearHighlights)

// ── Add Comment ─────────────────────────────────────────

async function handleAddComment(fragmentIndex: number) {
  const form = commentForms.value[fragmentIndex]
  if (!form || !selectedPull.value) return

  const email = isAuthenticated.value && user.value ? user.value.email : form.email.trim()
  if (!email) { message.warning('请输入邮箱地址'); return }
  if (!form.content.trim()) { message.warning('请输入评论内容'); return }

  form.submitting = true
  try {
    await addComment(selectedPull.value.id, fragmentIndex, 0, form.content.trim(), email)
    message.success('评论已添加')
    form.content = ''
    pullComments.value = await fetchComments(selectedPull.value.id)
  } catch {
    message.error('评论失败，请重试')
  } finally {
    form.submitting = false
  }
}

// ── Apply (Admin) ───────────────────────────────────────

const applying = ref(false)

async function handleApply() {
  if (!selectedPull.value) return
  applying.value = true
  try {
    await applyPull(selectedPull.value.id)
    message.success('PR 已合并！')
    closeDetail()
    emit('pr-applied')
  } catch {
    message.error('合并失败，请重试')
  } finally {
    applying.value = false
  }
}

// ── Helpers ─────────────────────────────────────────────

function truncate(text: string, max: number): string {
  return text.length > max ? text.slice(0, max) + '…' : text
}

// ── Init ────────────────────────────────────────────────

onMounted(fetchPulls)

defineExpose({ openCreateModal })
</script>

<template>
  <div class="pull-requests">
    <!-- Selection Action Bar -->
    <Transition name="pr-bar">
      <div v-if="selectedText" class="pr-selection-bar">
        <span class="pr-selection-bar__text">
          <span class="pr-selection-bar__icon">✂</span>
          已选择「{{ truncate(selectedText, 40) }}」
        </span>
        <n-button type="primary" size="small" @click="openCreateModal">
          创建 PR
        </n-button>
      </div>
    </Transition>

    <!-- Header -->
    <div class="pr-header">
      <h3 class="pr-header__title">
        <span class="pr-header__icon">⎇</span>
        Pull Requests
        <span v-if="pulls.length > 0" class="pr-header__count">{{ pulls.length }}</span>
      </h3>
      <n-select
        v-model:value="statusFilter"
        :options="statusOptions"
        size="small"
        style="width: 100px"
      />
    </div>

    <!-- Loading -->
    <div v-if="loading" class="pr-loading">
      <n-spin size="medium" />
    </div>

    <!-- Error -->
    <div v-else-if="error" class="pr-error">
      <p>{{ error }}</p>
      <n-button size="small" @click="fetchPulls">重试</n-button>
    </div>

    <!-- Empty -->
    <div v-else-if="filteredPulls.length === 0" class="pr-empty">
      <span class="pr-empty__icon">⎇</span>
      <p class="pr-empty__text">
        {{ statusFilter !== 'all' ? '没有符合筛选条件的 PR' : '暂无 Pull Request' }}
      </p>
    </div>

    <!-- PR List -->
    <div v-else class="pr-list">
      <div
        v-for="pull in filteredPulls"
        :key="pull.id"
        class="pr-card"
        :class="{ 'pr-card--selected': selectedPull?.id === pull.id }"
        @click="openDetail(pull)"
      >
        <div class="pr-card__header">
          <span class="pr-card__status" :style="{ color: getStatusColor(pull.status) }">●</span>
          <span class="pr-card__email">{{ pull.user_email }}</span>
          <span class="pr-card__date">{{ formatDate(pull.created_at) }}</span>
        </div>
        <p v-if="pull.message" class="pr-card__message">{{ pull.message }}</p>
        <div class="pr-card__footer">
          <span class="pr-card__fragments">{{ pull.fragments.length }} 个修改</span>
          <span
            class="pr-card__badge"
            :style="{
              color: getStatusColor(pull.status),
              backgroundColor: getStatusColor(pull.status) + '15',
              borderColor: getStatusColor(pull.status) + '30',
            }"
          >
            {{ getStatusLabel(pull.status) }}
          </span>
          <span class="pr-card__id">#{{ pull.id }}</span>
        </div>
      </div>
    </div>

    <!-- Create PR Modal -->
    <div v-if="showCreateModal" class="pr-overlay" @click.self="cancelCreate">
      <div class="pr-modal">
        <h3 class="pr-modal__title">创建 Pull Request</h3>
        <p class="pr-modal__desc">对选中的文本提出修改建议。</p>

        <div class="pr-modal__fields">
          <div class="form-field">
            <label class="form-field__label">已选择的文本</label>
            <div class="pr-modal__selected-text">{{ selectedText }}</div>
          </div>

          <div class="form-field">
            <label class="form-field__label" for="pr-replacement">替换为</label>
            <n-input
              id="pr-replacement"
              v-model:value="createReplacement"
              type="textarea"
              :rows="3"
              :disabled="creating"
              placeholder="输入替换内容..."
            />
          </div>

          <div class="form-field">
            <label class="form-field__label" for="pr-desc">说明（可选）</label>
            <n-input
              id="pr-desc"
              v-model:value="createDescription"
              :disabled="creating"
              placeholder="解释为什么需要这个修改"
            />
          </div>

          <div class="form-field">
            <label class="form-field__label" for="pr-message">PR 描述（可选）</label>
            <n-input
              id="pr-message"
              v-model:value="createMessage"
              type="textarea"
              :rows="2"
              :disabled="creating"
              placeholder="总体描述..."
            />
          </div>

          <template v-if="isAuthenticated">
            <p class="pr-modal__user-email">将以 {{ user?.email }} 身份提交</p>
          </template>
          <template v-else>
            <div class="form-field">
              <label class="form-field__label" for="pr-email">邮箱 *</label>
              <n-input
                id="pr-email"
                v-model:value="createEmail"
                :disabled="creating"
                placeholder="your@email.com"
              />
            </div>
          </template>
        </div>

        <div class="pr-modal__actions">
          <n-button @click="cancelCreate" :disabled="creating">取消</n-button>
          <n-button type="primary" :loading="creating" @click="handleCreate">提交 PR</n-button>
        </div>
      </div>
    </div>

    <!-- PR Detail Modal -->
    <div v-if="selectedPull" class="pr-overlay" @click.self="closeDetail">
      <div class="pr-detail-modal">
        <div class="pr-detail__header">
          <h3 class="pr-detail__title">
            <span class="pr-detail__status-dot" :style="{ color: getStatusColor(selectedPull.status) }">●</span>
            PR #{{ selectedPull.id }}
          </h3>
          <button class="pr-detail__close" @click="closeDetail">×</button>
        </div>

        <div class="pr-detail__meta">
          <span class="pr-detail__author">{{ selectedPull.user_email }}</span>
          <span class="pr-detail__date">{{ formatDate(selectedPull.created_at) }}</span>
          <span
            class="pr-detail__badge"
            :style="{
              color: getStatusColor(selectedPull.status),
              backgroundColor: getStatusColor(selectedPull.status) + '15',
              borderColor: getStatusColor(selectedPull.status) + '30',
            }"
          >
            {{ getStatusLabel(selectedPull.status) }}
          </span>
        </div>

        <p v-if="selectedPull.message" class="pr-detail__message">{{ selectedPull.message }}</p>

        <!-- Loading comments -->
        <div v-if="detailLoading" class="pr-detail__loading">
          <n-spin size="small" />
        </div>

        <!-- Fragments -->
        <div class="pr-fragments">
          <div v-for="(frag, i) in selectedPull.fragments" :key="i" class="fragment-card">
            <div class="fragment-card__header">
              <span class="fragment-card__index">片段 {{ i + 1 }}</span>
              <span class="fragment-card__position">
                L{{ frag.start_line }}:C{{ frag.start_col }} → L{{ frag.end_line }}:C{{ frag.end_col }}
              </span>
            </div>

            <p v-if="frag.description" class="fragment-card__desc">{{ frag.description }}</p>

            <div class="fragment-card__diff">
              <div class="fragment-card__side">
                <span class="fragment-card__label">原文</span>
                <code class="fragment-card__code">{{ getOriginalText(frag) || '—' }}</code>
              </div>
              <span class="fragment-card__arrow">→</span>
              <div class="fragment-card__side">
                <span class="fragment-card__label">替换为</span>
                <code class="fragment-card__code fragment-card__code--replacement">{{ frag.replacement }}</code>
              </div>
            </div>

            <!-- Fragment Comments -->
            <div class="fragment-comments">
              <div
                v-for="comment in getFragmentComments(i)"
                :key="comment.id"
                class="fragment-comment"
              >
                <div class="fragment-comment__header">
                  <span class="fragment-comment__author">{{ comment.user_email }}</span>
                  <span class="fragment-comment__date">{{ formatDate(comment.created_at) }}</span>
                </div>
                <p class="fragment-comment__content">{{ comment.content }}</p>
              </div>

              <div class="fragment-comment-form">
                <template v-if="isAuthenticated">
                  <p class="fragment-comment-form__user-email">以 {{ user?.email }} 身份评论</p>
                </template>
                <template v-else>
                  <n-input
                    v-model:value="getCommentForm(i).email"
                    placeholder="邮箱"
                    size="small"
                    :disabled="getCommentForm(i).submitting"
                  />
                </template>
                <n-input
                  v-model:value="getCommentForm(i).content"
                  type="textarea"
                  :rows="2"
                  placeholder="对此片段发表评论..."
                  size="small"
                  :disabled="getCommentForm(i).submitting"
                />
                <n-button
                  size="small"
                  type="primary"
                  :loading="getCommentForm(i).submitting"
                  @click="handleAddComment(i)"
                >
                  发送评论
                </n-button>
              </div>
            </div>
          </div>
        </div>

        <!-- Apply (Admin Only) -->
        <div v-if="isAuthenticated && selectedPull.status === 'open'" class="pr-detail__actions">
          <n-button type="primary" :loading="applying" @click="handleApply">
            合并 PR
          </n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ── Container ──────────────────────────────────────── */
.pull-requests {
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border);
}

/* ── Selection Action Bar ───────────────────────────── */
.pr-selection-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  margin-bottom: var(--space-4);
  background: var(--color-accent-light);
  border: 1px solid var(--color-accent);
  border-radius: var(--radius-md);
}

.pr-selection-bar__text {
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.pr-selection-bar__icon {
  flex-shrink: 0;
}

/* Transition for selection bar */
.pr-bar-enter-active,
.pr-bar-leave-active {
  transition: all var(--transition-fast);
}
.pr-bar-enter-from,
.pr-bar-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* ── Header ─────────────────────────────────────────── */
.pr-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.pr-header__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.pr-header__icon {
  font-size: var(--text-2xl);
  color: var(--color-accent);
}

.pr-header__count {
  font-family: var(--font-body);
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-tertiary);
  background: var(--color-bg-sunken);
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
}

/* ── States ─────────────────────────────────────────── */
.pr-loading,
.pr-error,
.pr-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-8) var(--space-4);
  gap: var(--space-3);
}

.pr-error p {
  color: var(--color-error);
  font-size: var(--text-sm);
}

.pr-empty__icon {
  font-size: var(--text-4xl);
  color: var(--color-border);
}

.pr-empty__text {
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

/* ── PR List ────────────────────────────────────────── */
.pr-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

/* ── PR Card ────────────────────────────────────────── */
.pr-card {
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  padding: var(--space-4);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.pr-card:hover {
  border-color: var(--color-accent-light);
  box-shadow: var(--shadow-sm);
}

.pr-card--selected {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 1px var(--color-accent);
}

.pr-card__header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}

.pr-card__status {
  font-size: var(--text-sm);
  line-height: 1;
}

.pr-card__email {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.pr-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin-left: auto;
}

.pr-card__message {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin-bottom: var(--space-3);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.pr-card__footer {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.pr-card__fragments {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.pr-card__badge {
  font-size: var(--text-xs);
  font-weight: 500;
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  border: 1px solid;
}

.pr-card__id {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
  margin-left: auto;
}

/* ── Overlay ────────────────────────────────────────── */
.pr-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: overlayFadeIn var(--transition-fast) ease-out;
  backdrop-filter: blur(4px);
}

@keyframes overlayFadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* ── Create Modal ───────────────────────────────────── */
.pr-modal {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 520px;
  max-height: 85vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

@keyframes modalSlideIn {
  from { opacity: 0; transform: translateY(16px) scale(0.98); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.pr-modal__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.pr-modal__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-6);
  line-height: 1.5;
}

.pr-modal__fields {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.pr-modal__selected-text {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-accent);
  background: var(--color-bg-sunken);
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-md);
  border-left: 3px solid var(--color-accent);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.pr-modal__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

.pr-modal__user-email {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  margin: 0;
}

/* ── Form Field ─────────────────────────────────────── */
.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.form-field__label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

/* ── Detail Modal ───────────────────────────────────── */
.pr-detail-modal {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  width: 90%;
  max-width: 600px;
  max-height: 85vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

.pr-detail__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.pr-detail__title {
  font-family: var(--font-display);
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.pr-detail__status-dot {
  font-size: var(--text-sm);
}

.pr-detail__close {
  background: none;
  border: none;
  font-size: var(--text-2xl);
  color: var(--color-text-tertiary);
  cursor: pointer;
  padding: 0;
  line-height: 1;
  transition: color var(--transition-fast);
}

.pr-detail__close:hover {
  color: var(--color-text-primary);
}

.pr-detail__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
  flex-wrap: wrap;
}

.pr-detail__author {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.pr-detail__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.pr-detail__badge {
  font-size: var(--text-xs);
  font-weight: 500;
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  border: 1px solid;
}

.pr-detail__message {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin-bottom: var(--space-4);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-sunken);
  border-radius: var(--radius-md);
}

.pr-detail__loading {
  display: flex;
  justify-content: center;
  padding: var(--space-4);
}

.pr-detail__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-6);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-border);
}

/* ── Fragments ──────────────────────────────────────── */
.pr-fragments {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.fragment-card {
  background: var(--color-bg);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  padding: var(--space-4);
}

.fragment-card__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-2);
}

.fragment-card__index {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--color-accent);
  background: var(--color-accent-light);
  padding: 1px var(--space-2);
  border-radius: var(--radius-sm);
}

.fragment-card__position {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.fragment-card__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  font-style: italic;
  margin-bottom: var(--space-3);
  line-height: 1.5;
}

.fragment-card__diff {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.fragment-card__side {
  flex: 1;
  min-width: 0;
}

.fragment-card__label {
  display: block;
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--space-1);
}

.fragment-card__code {
  display: block;
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  background: var(--color-bg-sunken);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.fragment-card__code--replacement {
  color: var(--color-success);
  background: rgba(74, 124, 89, 0.08);
  border: 1px solid rgba(74, 124, 89, 0.15);
}

.fragment-card__arrow {
  flex-shrink: 0;
  color: var(--color-text-tertiary);
  font-size: var(--text-lg);
  padding-top: var(--space-5);
}

/* ── Fragment Comments ──────────────────────────────── */
.fragment-comments {
  border-top: 1px solid var(--color-border-light);
  padding-top: var(--space-3);
}

.fragment-comment {
  padding: var(--space-2) 0;
}

.fragment-comment + .fragment-comment {
  border-top: 1px solid var(--color-border-light);
}

.fragment-comment__header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-1);
}

.fragment-comment__author {
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-text-primary);
}

.fragment-comment__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin-left: auto;
}

.fragment-comment__content {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.5;
  margin: 0;
}

.fragment-comment-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: var(--space-3);
  padding-top: var(--space-3);
  border-top: 1px solid var(--color-border-light);
}

.fragment-comment-form__user-email {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin: 0;
}

/* ── Fragment Highlight (injected into post body) ───── */
:global(.pr-fragment-highlight) {
  background: rgba(196, 93, 62, 0.15);
  border-bottom: 2px solid var(--color-accent);
  border-radius: 2px;
  padding: 1px 0;
  transition: background var(--transition-fast);
}

:global(.pr-fragment-highlight:hover) {
  background: rgba(196, 93, 62, 0.25);
}

/* ── Responsive ─────────────────────────────────────── */
@media (max-width: 767px) {
  .pr-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }

  .pr-modal,
  .pr-detail-modal {
    padding: var(--space-6);
    margin: var(--space-4);
  }

  .pr-detail__meta {
    flex-direction: column;
    align-items: flex-start;
  }

  .pr-detail__actions {
    flex-wrap: wrap;
  }

  .fragment-card__diff {
    flex-direction: column;
    gap: var(--space-2);
  }

  .fragment-card__arrow {
    transform: rotate(90deg);
    padding: 0;
    text-align: center;
  }
}
</style>
