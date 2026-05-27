<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { NSpin, NButton, NInput, NSelect, useMessage } from 'naive-ui'
import { usePullRequests } from '../composables/usePullRequests'
import type { PullRequest } from '../composables/usePullRequests'

const props = defineProps<{
  postId: number | string
}>()

const message = useMessage()

const {
  pulls,
  loading,
  error,
  fetchPulls,
  createPull,
  updatePull,
  addComment,
  formatDate,
  getStatusColor,
  getStatusLabel,
} = usePullRequests()

// 创建 PR 表单状态
const showCreateForm = ref(false)
const newPrEmail = ref('')
const newPrContent = ref('')
const creating = ref(false)

// PR 详情视图状态
const selectedPull = ref<PullRequest | null>(null)
const showComments = ref(false)
const comments = ref<Array<{ id: number; user_email: string; content: string; created_at: string }>>([])

// 评论表单状态
const commentEmail = ref('')
const commentContent = ref('')
const addingComment = ref(false)

// 状态筛选
const statusFilter = ref<string>('all')
const statusOptions = [
  { label: '全部', value: 'all' },
  { label: '开放', value: 'open' },
  { label: '已关闭', value: 'closed' },
  { label: '已合并', value: 'merged' },
]

// 获取筛选后的 PR 列表
const filteredPulls = ref<PullRequest[]>([])

watch([pulls, statusFilter], () => {
  if (statusFilter.value && statusFilter.value !== 'all') {
    filteredPulls.value = pulls.value.filter(p => p.status === statusFilter.value)
  } else {
    filteredPulls.value = [...pulls.value]
  }
}, { immediate: true })

// 初始化
onMounted(async () => {
  await fetchPulls(props.postId)
})

// 打开创建 PR 表单
function openCreateForm() {
  showCreateForm.value = true
  newPrEmail.value = ''
  newPrContent.value = ''
}

// 取消创建
function cancelCreate() {
  showCreateForm.value = false
  newPrEmail.value = ''
  newPrContent.value = ''
}

// 提交创建 PR
async function handleCreatePull() {
  if (!newPrEmail.value.trim()) {
    message.warning('请输入邮箱地址')
    return
  }
  if (!newPrContent.value.trim()) {
    message.warning('请输入 PR 内容')
    return
  }

  creating.value = true
  try {
    const result = await createPull(props.postId, {
      user_email: newPrEmail.value.trim(),
      content: newPrContent.value.trim(),
    })

    if (result.success) {
      message.success('PR 创建成功！')
      showCreateForm.value = false
      newPrEmail.value = ''
      newPrContent.value = ''
    } else {
      message.error(result.error || '创建失败，请重试')
    }
  } catch {
    message.error('创建失败，请重试')
  } finally {
    creating.value = false
  }
}

// 查看 PR 详情
function viewPullDetail(pull: PullRequest) {
  selectedPull.value = pull
  showComments.value = true
  commentEmail.value = ''
  commentContent.value = ''
}

// 关闭 PR 详情
function closePullDetail() {
  selectedPull.value = null
  showComments.value = false
  comments.value = []
}

// 更新 PR 状态
async function handleUpdateStatus(pullId: number, status: 'open' | 'closed' | 'merged') {
  const result = await updatePull(pullId, { status })
  if (result.success) {
    message.success(`PR 状态已更新为 ${getStatusLabel(status)}`)
    if (selectedPull.value?.id === pullId && result.data) {
      selectedPull.value = result.data as any
    }
  } else {
    message.error(result.error || '更新失败')
  }
}

// 添加评论
async function handleAddComment() {
  if (!selectedPull.value) return
  
  if (!commentEmail.value.trim()) {
    message.warning('请输入邮箱地址')
    return
  }
  if (!commentContent.value.trim()) {
    message.warning('请输入评论内容')
    return
  }

  addingComment.value = true
  try {
    const result = await addComment(selectedPull.value.id, {
      user_email: commentEmail.value.trim(),
      content: commentContent.value.trim(),
    })

    if (result.success && result.data) {
      comments.value.push(result.data)
      commentContent.value = ''
      message.success('评论添加成功！')
    } else {
      message.error(result.error || '评论失败，请重试')
    }
  } catch {
    message.error('评论失败，请重试')
  } finally {
    addingComment.value = false
  }
}
</script>

<template>
  <div class="pull-requests">
    <!-- 标题栏 -->
    <div class="pr-header">
      <h3 class="pr-header__title">
        <span class="pr-header__icon">⎇</span>
        Pull Requests
        <span v-if="pulls.length > 0" class="pr-header__count">{{ pulls.length }}</span>
      </h3>
      <div class="pr-header__actions">
        <n-select
          v-model:value="statusFilter"
          :options="statusOptions"
          placeholder="筛选状态"
          size="small"
          style="width: 120px;"
        />
        <n-button type="primary" size="small" @click="openCreateForm">
          创建 PR
        </n-button>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="pr-loading">
      <n-spin size="medium" />
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="pr-error">
      <p>{{ error }}</p>
      <n-button size="small" @click="fetchPulls(postId)">重试</n-button>
    </div>

    <!-- 空状态 -->
    <div v-else-if="filteredPulls.length === 0" class="pr-empty">
      <span class="pr-empty__icon">⎇</span>
      <p class="pr-empty__text">
        {{ statusFilter !== 'all' ? '没有符合筛选条件的 PR' : '暂无 Pull Request' }}
      </p>
      <n-button v-if="statusFilter === 'all'" type="primary" size="small" @click="openCreateForm">
        创建第一个 PR
      </n-button>
    </div>

    <!-- PR 列表 -->
    <div v-else class="pr-list">
      <div
        v-for="pull in filteredPulls"
        :key="pull.id"
        class="pr-card"
        :class="{ 'pr-card--selected': selectedPull?.id === pull.id }"
        @click="viewPullDetail(pull)"
      >
        <div class="pr-card__header">
          <span
            class="pr-card__status"
            :style="{ color: getStatusColor(pull.status) }"
          >
            ●
          </span>
          <span class="pr-card__email">{{ pull.user_email }}</span>
          <span class="pr-card__date">{{ formatDate(pull.created_at) }}</span>
        </div>
        <p class="pr-card__content">{{ pull.content }}</p>
        <div class="pr-card__footer">
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

    <!-- 创建 PR 弹窗 -->
    <div v-if="showCreateForm" class="pr-overlay" @click.self="cancelCreate">
      <div class="pr-modal">
        <h3 class="pr-modal__title">创建 Pull Request</h3>
        <p class="pr-modal__desc">
          提交你的建议、修改或反馈，作者可以查看并决定是否合并。
        </p>

        <div class="pr-modal__fields">
          <div class="form-field">
            <label class="form-field__label" for="pr-email">邮箱地址 *</label>
            <n-input
              id="pr-email"
              v-model:value="newPrEmail"
              type="text"
              placeholder="your@email.com"
              :disabled="creating"
            />
          </div>

          <div class="form-field">
            <label class="form-field__label" for="pr-content">PR 内容 *</label>
            <n-input
              id="pr-content"
              v-model:value="newPrContent"
              type="textarea"
              placeholder="描述你的修改建议或反馈..."
              :rows="4"
              :disabled="creating"
            />
          </div>
        </div>

        <div class="pr-modal__actions">
          <n-button @click="cancelCreate" :disabled="creating">
            取消
          </n-button>
          <n-button
            type="primary"
            :loading="creating"
            @click="handleCreatePull"
          >
            提交 PR
          </n-button>
        </div>
      </div>
    </div>

    <!-- PR 详情弹窗 -->
    <div v-if="showComments && selectedPull" class="pr-overlay" @click.self="closePullDetail">
      <div class="pr-detail-modal">
        <div class="pr-detail__header">
          <h3 class="pr-detail__title">
            <span
              class="pr-detail__status-dot"
              :style="{ color: getStatusColor(selectedPull.status) }"
            >
              ●
            </span>
            PR #{{ selectedPull.id }}
          </h3>
          <button class="pr-detail__close" @click="closePullDetail">×</button>
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

        <div class="pr-detail__content">
          {{ selectedPull.content }}
        </div>

        <!-- 状态操作 -->
        <div class="pr-detail__actions">
          <n-button
            v-if="selectedPull.status !== 'open'"
            size="small"
            @click="handleUpdateStatus(selectedPull.id, 'open')"
          >
            重新开放
          </n-button>
          <n-button
            v-if="selectedPull.status !== 'closed'"
            size="small"
            @click="handleUpdateStatus(selectedPull.id, 'closed')"
          >
            关闭
          </n-button>
          <n-button
            v-if="selectedPull.status !== 'merged'"
            type="primary"
            size="small"
            @click="handleUpdateStatus(selectedPull.id, 'merged')"
          >
            合并
          </n-button>
        </div>

        <!-- 评论区 -->
        <div class="pr-comments">
          <h4 class="pr-comments__title">评论</h4>
          
          <div v-if="comments.length === 0" class="pr-comments__empty">
            暂无评论
          </div>

          <div v-else class="pr-comments__list">
            <div
              v-for="comment in comments"
              :key="comment.id"
              class="comment-item"
            >
              <div class="comment-item__header">
                <span class="comment-item__author">{{ comment.user_email }}</span>
                <span class="comment-item__date">{{ formatDate(comment.created_at) }}</span>
              </div>
              <p class="comment-item__content">{{ comment.content }}</p>
            </div>
          </div>

          <!-- 添加评论表单 -->
          <div class="comment-form">
            <div class="form-field">
              <label class="form-field__label" for="comment-email">邮箱 *</label>
              <n-input
                id="comment-email"
                v-model:value="commentEmail"
                type="text"
                placeholder="your@email.com"
                size="small"
                :disabled="addingComment"
              />
            </div>
            <div class="form-field">
              <label class="form-field__label" for="comment-content">评论 *</label>
              <n-input
                id="comment-content"
                v-model:value="commentContent"
                type="textarea"
                placeholder="写下你的评论..."
                :rows="2"
                size="small"
                :disabled="addingComment"
              />
            </div>
            <n-button
              type="primary"
              size="small"
              :loading="addingComment"
              @click="handleAddComment"
            >
              发送评论
            </n-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* --- PR 组件容器 --- */
.pull-requests {
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border);
}

/* --- 标题栏 --- */
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

.pr-header__actions {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

/* --- 加载/错误/空状态 --- */
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

/* --- PR 列表 --- */
.pr-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

/* --- PR 卡片 --- */
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

.pr-card__content {
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

/* --- 弹窗遮罩 --- */
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

/* --- 创建 PR 弹窗 --- */
.pr-modal {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 480px;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: translateY(16px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
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

.pr-modal__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

/* --- 表单字段 --- */
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

/* --- PR 详情弹窗 --- */
.pr-detail-modal {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  width: 90%;
  max-width: 560px;
  max-height: 80vh;
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

.pr-detail__content {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.7;
  margin-bottom: var(--space-4);
  padding: var(--space-4);
  background: var(--color-bg-sunken);
  border-radius: var(--radius-md);
}

.pr-detail__actions {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-6);
  padding-bottom: var(--space-4);
  border-bottom: 1px solid var(--color-border);
}

/* --- 评论区 --- */
.pr-comments {
  margin-top: var(--space-4);
}

.pr-comments__title {
  font-family: var(--font-display);
  font-size: var(--text-base);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-4);
}

.pr-comments__empty {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  text-align: center;
  padding: var(--space-4) 0;
}

.pr-comments__list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

/* --- 评论项 --- */
.comment-item {
  padding: var(--space-3);
  background: var(--color-bg-sunken);
  border-radius: var(--radius-md);
}

.comment-item__header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}

.comment-item__author {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.comment-item__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin-left: auto;
}

.comment-item__content {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin: 0;
}

/* --- 评论表单 --- */
.comment-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-border);
}

/* --- 响应式 --- */
@media (max-width: 767px) {
  .pr-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }

  .pr-header__actions {
    width: 100%;
    justify-content: space-between;
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
}
</style>
