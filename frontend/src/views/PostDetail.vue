<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NSpin, NButton, NInput, NAlert, NTag, NSpace, useMessage } from 'naive-ui'
import { usePosts } from '../composables/usePosts'
import { useTags, type Tag } from '../composables/useTags'
import { useComments } from '../composables/useComments'
import { useRevisions, type Revision, type DiffResponse } from '../composables/useRevisions'
import { useAuth } from '../composables/useAuth'
import PullRequest from '../components/PullRequest.vue'
import MarkdownRenderer from '../components/MarkdownRenderer.vue'

const route = useRoute()
const router = useRouter()
const message = useMessage()

const {
  currentPost,
  postTodos,
  loading,
  error,
  fetchPost,
  fetchPostTodos,
  fetchTodoSubscriberCounts,
  subscribeToTodo,
  unsubscribeFromTodo,
  createSubscriber,
} = usePosts()

const {
  comments,
  submitting: commentSubmitting,
  fetchApprovedComments,
  submitComment,
  formatRelativeTime,
} = useComments()

const { fetchPostTags } = useTags()

const { user, isAuthenticated } = useAuth()

// 订阅表单状态
const subscriberEmail = ref('')
const subscriberName = ref('')
const subscribing = ref(false)
const showSubscribeForm = ref(false)
const activeTodoTitle = ref('')
const activeTodoId = ref<number | null>(null)
const todoSubscriberCounts = ref<Map<number, number>>(new Map())
const subscribedTodoIds = ref<Set<number>>(new Set())
const inlineEmailInputs = ref<Record<number, string>>({})
const inlineSubscribing = ref<Set<number>>(new Set())

// 评论表单状态
const commentName = ref('')
const commentEmail = ref('')
const commentContent = ref('')
const commentSubmitted = ref(false)
const referencedContent = ref('')

// 文章标签
const postTags = ref<Tag[]>([])

// 版本历史面板状态
const showRevisionPanel = ref(false)
const showDiffView = ref(false)
const showRollbackConfirm = ref(false)
const selectedRevision = ref<Revision | null>(null)
const diffData = ref<DiffResponse | null>(null)
const diffFromRev = ref<Revision | null>(null)
const diffToRev = ref<Revision | null>(null)
const rollbackTargetRev = ref<Revision | null>(null)
const rollbacking = ref(false)
const diffLoading = ref(false)

// Post body DOM ref (for PR fragment selection/highlighting)
const postBodyRef = ref<HTMLElement | null>(null)

// 路由中的文章 ID
const postId = computed(() => route.params.id as string)

// 修订版本管理
const {
  revisions,
  loading: revisionsLoading,
  fetchRevisions,
  rollback: rollbackRevision,
  getDiff,
  formatRelativeTime: formatRevisionTime,
} = useRevisions(postId)

// 格式化完整日期
function formatFullDate(isoDate: string): string {
  const date = new Date(isoDate)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long',
  })
}

// 处理待办订阅点击
async function handleTodoSubscribe(todoId: number, taskTitle: string) {
  if (isAuthenticated.value && user.value) {
    const result = await subscribeToTodo(todoId, undefined, user.value.id)
    if (result.success) {
      message.success('订阅成功！当待办事项更新时会通知您。')
      subscribedTodoIds.value.add(todoId)
      todoSubscriberCounts.value.set(todoId, (todoSubscriberCounts.value.get(todoId) || 0) + 1)
    } else if (result.error === 'already_subscribed') {
      message.warning('您已订阅此待办事项')
      subscribedTodoIds.value.add(todoId)
    } else {
      message.error(result.error || '订阅失败')
    }
  } else {
    activeTodoId.value = todoId
    activeTodoTitle.value = taskTitle
    showSubscribeForm.value = true
  }
}

async function handleInlineSubscribe(todoId: number) {
  const email = inlineEmailInputs.value[todoId]?.trim()
  if (!email) {
    message.warning('请输入邮箱地址')
    return
  }

  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(email)) {
    message.warning('请输入有效的邮箱地址')
    return
  }

  inlineSubscribing.value.add(todoId)
  const result = await subscribeToTodo(todoId, email)
  if (result.success) {
    message.success('订阅成功！')
    subscribedTodoIds.value.add(todoId)
    todoSubscriberCounts.value.set(todoId, (todoSubscriberCounts.value.get(todoId) || 0) + 1)
    inlineEmailInputs.value[todoId] = ''
  } else if (result.error === 'already_subscribed') {
    message.warning('您已订阅此待办事项')
    subscribedTodoIds.value.add(todoId)
  } else {
    message.error(result.error || '订阅失败')
  }
  inlineSubscribing.value.delete(todoId)
}

async function handleUnsubscribe(todoId: number) {
  const userId = user.value?.id
  const email = inlineEmailInputs.value[todoId]?.trim()

  const result = await unsubscribeFromTodo(todoId, email || undefined, userId)
  if (result.success) {
    message.success('已取消订阅')
    subscribedTodoIds.value.delete(todoId)
    todoSubscriberCounts.value.set(todoId, Math.max(0, (todoSubscriberCounts.value.get(todoId) || 0) - 1))
  } else {
    message.error(result.error || '取消订阅失败')
  }
}

function handleMarkdownTodoClick(taskTitle: string) {
  activeTodoId.value = null
  activeTodoTitle.value = taskTitle
  showSubscribeForm.value = true
}

// 提交订阅
async function handleSubscribe() {
  const email = isAuthenticated.value && user.value ? user.value.email : subscriberEmail.value.trim()
  if (!email) {
    message.warning('请输入邮箱地址')
    return
  }

  subscribing.value = true
  try {
    if (activeTodoId.value) {
      const result = isAuthenticated.value && user.value
        ? await subscribeToTodo(activeTodoId.value, undefined, user.value.id)
        : await subscribeToTodo(activeTodoId.value, email)
      if (result.success) {
        message.success('订阅成功！当待办事项更新时会通知您。')
        subscribedTodoIds.value.add(activeTodoId.value)
        todoSubscriberCounts.value.set(activeTodoId.value, (todoSubscriberCounts.value.get(activeTodoId.value) || 0) + 1)
      } else if (result.error === 'already_subscribed') {
        message.warning('您已订阅此待办事项')
        subscribedTodoIds.value.add(activeTodoId.value)
      } else {
        message.error(result.error || '订阅失败')
      }
    } else {
      const result = await createSubscriber({
        email,
        name: isAuthenticated.value && user.value ? user.value.name : (subscriberName.value.trim() || undefined),
      })
      if (result.success) {
        message.success('订阅成功！当待办事项更新时会通知您。')
      } else {
        message.error(result.error || '订阅失败，请重试')
      }
    }
    showSubscribeForm.value = false
    subscriberEmail.value = ''
    subscriberName.value = ''
  } catch {
    message.error('订阅失败，请重试')
  } finally {
    subscribing.value = false
  }
}

// 取消订阅
function cancelSubscribe() {
  showSubscribeForm.value = false
  subscriberEmail.value = ''
  subscriberName.value = ''
}

// 提交评论
async function handleSubmitComment() {
  const authorName = isAuthenticated.value && user.value ? user.value.name : commentName.value.trim()
  const authorEmail = isAuthenticated.value && user.value ? user.value.email : commentEmail.value.trim() || undefined

  if (!authorName) {
    message.warning('请输入您的称呼')
    return
  }
  if (!commentContent.value.trim()) {
    message.warning('请输入评论内容')
    return
  }

  const result = await submitComment(postId.value, {
    author_name: authorName,
    author_email: authorEmail,
    content: commentContent.value.trim(),
    referenced_content: referencedContent.value.trim() || undefined,
  })

  if (result.success) {
    message.success('评论已提交，等待审核')
    commentSubmitted.value = true
    commentName.value = ''
    commentEmail.value = ''
    commentContent.value = ''
    referencedContent.value = ''
  } else {
    message.error(result.error || '评论提交失败，请重试')
  }
}

// 引用选中文本
function quoteSelectedText() {
  const selection = window.getSelection()
  if (!selection || selection.isCollapsed || !selection.toString().trim()) {
    message.warning('请先选中文章中的文本')
    return
  }

  // Verify the selection is within the post body
  const range = selection.getRangeAt(0)
  if (postBodyRef.value && postBodyRef.value.contains(range.commonAncestorContainer)) {
    referencedContent.value = selection.toString().trim()
    message.success('已引用选中内容')
  } else {
    message.warning('请选中文章中的文本')
  }
}

// 清除引用内容
function clearReferencedContent() {
  referencedContent.value = ''
}

// 返回
function goBack() {
  router.push('/')
}

// --- 版本历史功能 ---

// 打开版本历史面板
async function openRevisionPanel() {
  showRevisionPanel.value = true
  await fetchRevisions()
}

// 关闭版本历史面板
function closeRevisionPanel() {
  showRevisionPanel.value = false
  showDiffView.value = false
  diffData.value = null
  selectedRevision.value = null
}

// 查看两个版本的 Diff
async function viewDiff(fromRev: Revision, toRev: Revision) {
  diffFromRev.value = fromRev
  diffToRev.value = toRev
  diffLoading.value = true
  showDiffView.value = true

  const result = await getDiff(fromRev.id, toRev.id)
  diffData.value = result
  diffLoading.value = false
}

// 关闭 Diff 视图
function closeDiffView() {
  showDiffView.value = false
  diffData.value = null
  diffFromRev.value = null
  diffToRev.value = null
}

// 确认回滚
function confirmRollback(rev: Revision) {
  rollbackTargetRev.value = rev
  showRollbackConfirm.value = true
}

// 执行回滚
async function handleRollback() {
  if (!rollbackTargetRev.value) return

  rollbacking.value = true
  const result = await rollbackRevision(rollbackTargetRev.value.id)
  rollbacking.value = false

  if (result.success) {
    message.success(`已回滚到版本 ${rollbackTargetRev.value.version}`)
    showRollbackConfirm.value = false
    rollbackTargetRev.value = null
    // 刷新文章内容
    await fetchPost(postId.value)
  } else {
    message.error(result.error || '回滚失败，请重试')
  }
}

// 取消回滚
function cancelRollback() {
  showRollbackConfirm.value = false
  rollbackTargetRev.value = null
}

// PR applied — refresh post content
async function handlePRApplied() {
  await fetchPost(postId.value)
}

// 解析 Diff 文本为行数据
interface DiffLine {
  type: 'add' | 'delete' | 'context'
  content: string
  oldLine: number | null
  newLine: number | null
}

function parseDiff(diffText: string): DiffLine[] {
  const lines = diffText.split('\n')
  const result: DiffLine[] = []
  let oldLine = 1
  let newLine = 1

  for (const line of lines) {
    if (line.startsWith('+')) {
      result.push({ type: 'add', content: line.slice(1), oldLine: null, newLine: newLine++ })
    } else if (line.startsWith('-')) {
      result.push({ type: 'delete', content: line.slice(1), oldLine: oldLine++, newLine: null })
    } else {
      result.push({ type: 'context', content: line.slice(1), oldLine: oldLine++, newLine: newLine++ })
    }
  }

  return result
}

onMounted(async () => {
  await fetchPost(postId.value)
  await fetchPostTodos(postId.value)
  await fetchApprovedComments(postId.value)
  postTags.value = await fetchPostTags(postId.value)

  const counts = await fetchTodoSubscriberCounts()
  todoSubscriberCounts.value = counts

  for (const todo of postTodos.value) {
    const count = counts.get(todo.id)
    if (count !== undefined) {
      todo.subscriber_count = count
    }
  }
})
</script>

<template>
  <div class="post-detail">
    <!-- Back Button -->
    <button class="back-button" @click="goBack">
      <span class="back-button__icon">←</span>
      <span class="back-button__text">返回文章列表</span>
    </button>

    <!-- Loading State -->
    <div v-if="loading" class="post-detail__loading">
      <n-spin size="large" />
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="post-detail__error">
      <h2>加载失败</h2>
      <p>{{ error }}</p>
      <n-button @click="fetchPost(postId)">重试</n-button>
    </div>

    <!-- Post Content -->
    <article v-else-if="currentPost" class="post-content">
      <!-- Post Header -->
      <header class="post-header">
        <div class="post-header__meta">
          <time class="post-header__date" :datetime="currentPost.created_at">
            {{ formatFullDate(currentPost.created_at) }}
          </time>
          <span v-if="currentPost.status === 'draft'" class="post-header__badge">
            草稿
          </span>
          <button
            v-if="isAuthenticated"
            class="post-header__edit-btn"
            @click="router.push(`/publish/${postId}`)"
          >
            <span class="post-header__edit-icon">✎</span>
            编辑
          </button>
          <button class="post-header__revision-btn" @click="openRevisionPanel">
            <span class="post-header__revision-icon">⎇</span>
            版本历史
          </button>
        </div>

        <h1 class="post-header__title">{{ currentPost.title }}</h1>

        <p v-if="currentPost.excerpt" class="post-header__excerpt">
          {{ currentPost.excerpt }}
        </p>

        <!-- Tag Badges -->
        <div v-if="postTags.length > 0" class="post-header__tags">
          <n-space :size="6">
            <n-tag
              v-for="tag in postTags"
              :key="tag.id"
              size="small"
              :bordered="false"
              round
            >
              {{ tag.name }}
            </n-tag>
          </n-space>
        </div>
      </header>

      <!-- Cover Image -->
      <div v-if="currentPost.cover_image" class="post-cover">
        <img :src="currentPost.cover_image" :alt="currentPost.title" />
      </div>

      <!-- Post Body -->
      <div ref="postBodyRef" class="post-body">
        <MarkdownRenderer :content="currentPost.content" @todo-click="handleMarkdownTodoClick" />
      </div>

      <!-- Todo Items Section -->
      <section v-if="postTodos.length > 0" class="todo-section">
        <h3 class="todo-section__title">待办事项</h3>
        <ul class="todo-list">
          <li
            v-for="todo in postTodos"
            :key="todo.id"
            class="todo-item"
            :class="{ 'todo-item--completed': todo.completed }"
          >
            <span class="todo-item__status">
              {{ todo.completed ? '✓' : '○' }}
            </span>
            <span class="todo-item__title">{{ todo.title }}</span>

            <span v-if="todoSubscriberCounts.has(todo.id)" class="todo-item__count">
              {{ todoSubscriberCounts.get(todo.id) }}人订阅
            </span>

            <template v-if="subscribedTodoIds.has(todo.id)">
              <button
                class="todo-item__unsubscribe"
                @click="handleUnsubscribe(todo.id)"
              >
                取消订阅
              </button>
            </template>
            <template v-else-if="isAuthenticated">
              <button
                class="todo-item__subscribe"
                @click="handleTodoSubscribe(todo.id, todo.title)"
              >
                订阅
              </button>
            </template>
            <template v-else>
              <div class="todo-item__inline-subscribe">
                <input
                  v-model="inlineEmailInputs[todo.id]"
                  class="todo-item__email-input"
                  type="email"
                  placeholder="邮箱"
                  @keyup.enter="handleInlineSubscribe(todo.id)"
                />
                <button
                  class="todo-item__subscribe"
                  :disabled="inlineSubscribing.has(todo.id)"
                  @click="handleInlineSubscribe(todo.id)"
                >
                  {{ inlineSubscribing.has(todo.id) ? '...' : '订阅' }}
                </button>
              </div>
            </template>
          </li>
        </ul>
      </section>

      <!-- Pull Requests Section -->
      <PullRequest
        :post-id="Number(postId)"
        :post-content="currentPost?.content"
        :post-body-el="postBodyRef"
        @pr-applied="handlePRApplied"
      />

      <!-- Comments Section -->
      <section class="comments-section">
        <h3 class="comments-section__title">评论</h3>

        <!-- Comment Form -->
        <div class="comment-form">
          <div v-if="commentSubmitted" class="comment-form__success">
            <n-alert type="success" :show-icon="true">
              评论已提交，等待管理员审核后将会显示。
            </n-alert>
          </div>

          <template v-else>
            <div class="comment-form__quote-action">
              <button class="quote-button" @click="quoteSelectedText">
                <span class="quote-button__icon">❝</span>
                引用选中内容
              </button>
            </div>

            <div v-if="referencedContent" class="comment-form__reference">
              <div class="reference-preview">
                <div class="reference-preview__header">
                  <span class="reference-preview__label">引用内容</span>
                  <button class="reference-preview__clear" @click="clearReferencedContent">×</button>
                </div>
                <blockquote class="reference-preview__content">{{ referencedContent }}</blockquote>
              </div>
            </div>

            <div class="comment-form__fields">
              <template v-if="isAuthenticated">
                <div class="comment-form__user-info">
                  <span class="comment-form__user-name">{{ user?.name }}</span>
                  <span class="comment-form__user-email">{{ user?.email }}</span>
                </div>
              </template>
              <template v-else>
                <div class="form-field">
                  <label class="form-field__label" for="comment-name">称呼 *</label>
                  <n-input
                    id="comment-name"
                    v-model:value="commentName"
                    type="text"
                    placeholder="您的称呼"
                    :disabled="commentSubmitting"
                  />
                </div>

                <div class="form-field">
                  <label class="form-field__label" for="comment-email">邮箱（可选）</label>
                  <n-input
                    id="comment-email"
                    v-model:value="commentEmail"
                    type="text"
                    placeholder="your@email.com"
                    :disabled="commentSubmitting"
                  />
                </div>
              </template>

              <div class="form-field">
                <label class="form-field__label" for="comment-content">评论内容 *</label>
                <n-input
                  id="comment-content"
                  v-model:value="commentContent"
                  type="textarea"
                  placeholder="写下您的评论..."
                  :rows="4"
                  :disabled="commentSubmitting"
                />
              </div>
            </div>

            <div class="comment-form__actions">
              <n-button
                type="primary"
                :loading="commentSubmitting"
                @click="handleSubmitComment"
              >
                提交评论
              </n-button>
            </div>
          </template>
        </div>

        <!-- Approved Comments List -->
        <div v-if="comments.length > 0" class="comments-list">
          <div
            v-for="comment in comments"
            :key="comment.id"
            class="comment-item"
          >
            <div class="comment-item__header">
              <span class="comment-item__author">{{ comment.author_name }}</span>
              <time class="comment-item__time" :datetime="comment.created_at">
                {{ formatRelativeTime(comment.created_at) }}
              </time>
            </div>
            <blockquote v-if="comment.referenced_content" class="comment-item__reference">
              {{ comment.referenced_content }}
            </blockquote>
            <p class="comment-item__content">{{ comment.content }}</p>
          </div>
        </div>

        <div v-else-if="comments.length === 0" class="comments-empty">
          <p class="comments-empty__text">暂无评论，来发表第一条吧</p>
        </div>
      </section>

      <!-- Subscribe Modal/Form -->
      <div v-if="showSubscribeForm" class="subscribe-overlay" @click.self="cancelSubscribe">
        <div class="subscribe-form">
          <h3 class="subscribe-form__title">订阅待办更新</h3>
          <p class="subscribe-form__desc">
            当「{{ activeTodoTitle }}」有更新时，我们会通过邮件通知您。
          </p>

          <div class="subscribe-form__fields">
            <template v-if="isAuthenticated">
              <p class="subscribe-form__user-email">将使用 {{ user?.email }} 订阅</p>
            </template>
            <template v-else>
              <div class="form-field">
                <label class="form-field__label" for="sub-email">邮箱地址 *</label>
                <n-input
                  id="sub-email"
                  v-model:value="subscriberEmail"
                  type="text"
                  placeholder="your@email.com"
                  :disabled="subscribing"
                  @keyup.enter="handleSubscribe"
                />
              </div>

              <div class="form-field">
                <label class="form-field__label" for="sub-name">称呼（可选）</label>
                <n-input
                  id="sub-name"
                  v-model:value="subscriberName"
                  type="text"
                  placeholder="您的称呼"
                  :disabled="subscribing"
                />
              </div>
            </template>
          </div>

          <div class="subscribe-form__actions">
            <n-button @click="cancelSubscribe" :disabled="subscribing">
              取消
            </n-button>
            <n-button
              type="primary"
              :loading="subscribing"
              @click="handleSubscribe"
            >
              订阅
            </n-button>
          </div>
        </div>
      </div>

      <!-- Version History Panel -->
      <div v-if="showRevisionPanel" class="revision-overlay" @click.self="closeRevisionPanel">
        <div class="revision-panel">
          <div class="revision-panel__header">
            <h3 class="revision-panel__title">
              <span class="revision-panel__icon">⎇</span>
              版本历史
            </h3>
            <button class="revision-panel__close" @click="closeRevisionPanel">×</button>
          </div>

          <!-- Loading -->
          <div v-if="revisionsLoading" class="revision-panel__loading">
            <n-spin size="medium" />
          </div>

          <!-- Empty -->
          <div v-else-if="revisions.length === 0" class="revision-panel__empty">
            <span class="revision-panel__empty-icon">⎇</span>
            <p>暂无版本历史记录</p>
          </div>

          <!-- Revision List -->
          <div v-else class="revision-list">
            <div
              v-for="(rev, index) in revisions"
              :key="rev.id"
              class="revision-item"
              :class="{ 'revision-item--selected': selectedRevision?.id === rev.id }"
            >
              <div class="revision-item__header">
                <span class="revision-item__version">v{{ rev.version }}</span>
                <span class="revision-item__time">{{ formatRevisionTime(rev.created_at) }}</span>
              </div>
              <div v-if="rev.title" class="revision-item__title">{{ rev.title }}</div>
              <div class="revision-item__actions">
                <n-button
                  v-if="index < revisions.length - 1"
                  size="tiny"
                  quaternary
                  @click="viewDiff(revisions[index + 1], rev)"
                >
                  查看 Diff
                </n-button>
                <n-button
                  size="tiny"
                  quaternary
                  type="warning"
                  @click="confirmRollback(rev)"
                >
                  回滚
                </n-button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Diff View Modal -->
      <div v-if="showDiffView" class="revision-overlay" @click.self="closeDiffView">
        <div class="diff-modal">
          <div class="diff-modal__header">
            <h3 class="diff-modal__title">
              版本对比
              <span v-if="diffFromRev && diffToRev" class="diff-modal__range">
                v{{ diffFromRev.version }} → v{{ diffToRev.version }}
              </span>
            </h3>
            <button class="diff-modal__close" @click="closeDiffView">×</button>
          </div>

          <!-- Diff Loading -->
          <div v-if="diffLoading" class="diff-modal__loading">
            <n-spin size="medium" />
          </div>

          <!-- Diff Content -->
          <div v-else-if="diffData" class="diff-content">
            <div class="diff-content__stats">
              <span class="diff-stat diff-stat--add">
                +{{ parseDiff(diffData.diff).filter(l => l.type === 'add').length }}
              </span>
              <span class="diff-stat diff-stat--delete">
                -{{ parseDiff(diffData.diff).filter(l => l.type === 'delete').length }}
              </span>
            </div>
            <div class="diff-content__lines">
              <div
                v-for="(line, i) in parseDiff(diffData.diff)"
                :key="i"
                class="diff-line"
                :class="{
                  'diff-line--add': line.type === 'add',
                  'diff-line--delete': line.type === 'delete',
                }"
              >
                <span class="diff-line__old-num">{{ line.oldLine ?? '' }}</span>
                <span class="diff-line__new-num">{{ line.newLine ?? '' }}</span>
                <span class="diff-line__marker">
                  {{ line.type === 'add' ? '+' : line.type === 'delete' ? '-' : ' ' }}
                </span>
                <span class="diff-line__content">{{ line.content }}</span>
              </div>
            </div>
          </div>

          <!-- Diff Error -->
          <div v-else class="diff-modal__error">
            <p>无法加载差异数据</p>
          </div>
        </div>
      </div>

      <!-- Rollback Confirmation Dialog -->
      <div v-if="showRollbackConfirm" class="revision-overlay" @click.self="cancelRollback">
        <div class="rollback-dialog">
          <h3 class="rollback-dialog__title">确认回滚</h3>
          <p class="rollback-dialog__desc">
            确定要回滚到版本 <strong>v{{ rollbackTargetRev?.version }}</strong> 吗？
          </p>
          <p class="rollback-dialog__warning">
            此操作会将文章内容恢复到该版本的状态，并创建一个新的修订版本。
          </p>
          <div class="rollback-dialog__actions">
            <n-button @click="cancelRollback" :disabled="rollbacking">
              取消
            </n-button>
            <n-button
              type="warning"
              :loading="rollbacking"
              @click="handleRollback"
            >
              确认回滚
            </n-button>
          </div>
        </div>
      </div>
    </article>
  </div>
</template>

<style scoped>
/* --- 文章详情布局 --- */
.post-detail {
  animation: fadeIn var(--transition-slow) ease-out;
  max-width: 720px;
  margin: 0 auto;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* --- 返回按钮 --- */
.back-button {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) 0;
  margin-bottom: var(--space-6);
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
  font-family: var(--font-body);
  transition: color var(--transition-fast);
}

.back-button:hover {
  color: var(--color-accent);
}

.back-button__icon {
  font-size: var(--text-lg);
  transition: transform var(--transition-fast);
}

.back-button:hover .back-button__icon {
  transform: translateX(-4px);
}

/* --- 加载/错误状态 --- */
.post-detail__loading,
.post-detail__error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  gap: var(--space-4);
}

.post-detail__error h2 {
  font-family: var(--font-display);
  color: var(--color-text-primary);
}

.post-detail__error p {
  color: var(--color-error);
}

/* --- 文章头部 --- */
.post-header {
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.post-header__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.post-header__date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.post-header__badge {
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-warning);
  background: rgba(196, 155, 62, 0.1);
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-header__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  line-height: 1.2;
  letter-spacing: -0.03em;
  margin-bottom: var(--space-4);
}

.post-header__excerpt {
  font-size: var(--text-lg);
  color: var(--color-text-secondary);
  font-style: italic;
  line-height: 1.6;
}

.post-header__tags {
  margin-top: var(--space-4);
}

/* --- 编辑按钮 --- */
.post-header__edit-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-3);
  background: transparent;
  color: var(--color-accent);
  border: 1px solid var(--color-accent);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.post-header__edit-btn:hover {
  background: var(--color-accent);
  color: white;
}

.post-header__edit-icon {
  font-size: var(--text-sm);
}

/* --- 版本历史按钮 --- */
.post-header__revision-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-3);
  background: transparent;
  color: var(--color-text-tertiary);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
  margin-left: auto;
}

.post-header__revision-btn:hover {
  color: var(--color-accent);
  border-color: var(--color-accent);
  background: var(--color-accent-light);
}

.post-header__revision-icon {
  font-size: var(--text-sm);
}

/* --- 封面图片 --- */
.post-cover {
  margin-bottom: var(--space-8);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-bg-sunken);
}

.post-cover img {
  width: 100%;
  height: auto;
  display: block;
  max-height: 400px;
  object-fit: cover;
}

/* --- 文章正文 --- */
.post-body {
  font-family: var(--font-body);
  font-size: var(--text-base);
  line-height: 1.8;
  color: var(--color-text-primary);
}

/* --- 待办事项区域 --- */
.todo-section {
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border);
}

.todo-section__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-4);
}

.todo-list {
  list-style: none;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.todo-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.todo-item:hover {
  border-color: var(--color-accent-light);
  box-shadow: var(--shadow-sm);
}

.todo-item--completed {
  opacity: 0.6;
}

.todo-item--completed .todo-item__title {
  text-decoration: line-through;
  color: var(--color-text-tertiary);
}

.todo-item__status {
  font-size: var(--text-lg);
  color: var(--color-accent);
  flex-shrink: 0;
}

.todo-item__title {
  flex: 1;
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.todo-item__subscribe {
  padding: var(--space-1) var(--space-3);
  background: transparent;
  color: var(--color-accent);
  border: 1px solid var(--color-accent);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.todo-item__subscribe:hover {
  background: var(--color-accent);
  color: white;
}

.todo-item__subscribe:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.todo-item__unsubscribe {
  padding: var(--space-1) var(--space-3);
  background: transparent;
  color: var(--color-text-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.todo-item__unsubscribe:hover {
  color: var(--color-error);
  border-color: var(--color-error);
}

.todo-item__count {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  flex-shrink: 0;
  padding: 0 var(--space-2);
}

.todo-item__inline-subscribe {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.todo-item__email-input {
  width: 140px;
  padding: var(--space-1) var(--space-2);
  font-size: var(--text-xs);
  font-family: var(--font-body);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-bg);
  color: var(--color-text-primary);
  outline: none;
  transition: border-color var(--transition-fast);
}

.todo-item__email-input:focus {
  border-color: var(--color-accent);
}

.todo-item__email-input::placeholder {
  color: var(--color-text-tertiary);
}

/* --- 评论区域 --- */
.comments-section {
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border);
}

.comments-section__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-6);
}

/* --- 评论表单 --- */
.comment-form {
  margin-bottom: var(--space-8);
  padding: var(--space-6);
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
}

.comment-form__success {
  margin-bottom: var(--space-4);
}

.comment-form__user-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
}

.comment-form__user-name {
  font-weight: 600;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
}

.comment-form__user-email {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.comment-form__fields {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.comment-form__actions {
  display: flex;
  justify-content: flex-end;
}

/* --- 引用按钮 --- */
.comment-form__quote-action {
  margin-bottom: var(--space-4);
}

.quote-button {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  background: transparent;
  color: var(--color-accent);
  border: 1px solid var(--color-accent);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.quote-button:hover {
  background: var(--color-accent);
  color: white;
}

.quote-button__icon {
  font-size: var(--text-lg);
  line-height: 1;
}

/* --- 引用预览 --- */
.comment-form__reference {
  margin-bottom: var(--space-4);
}

.reference-preview {
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
}

.reference-preview__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-2);
}

.reference-preview__label {
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.reference-preview__clear {
  background: none;
  border: none;
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  cursor: pointer;
  padding: 0;
  line-height: 1;
  transition: color var(--transition-fast);
}

.reference-preview__clear:hover {
  color: var(--color-error);
}

.reference-preview__content {
  margin: 0;
  padding: var(--space-2) var(--space-4);
  border-left: 3px solid var(--color-accent);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

/* --- 评论列表 --- */
.comments-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.comment-item {
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  transition: background-color var(--transition-fast);
}

.comment-item:hover {
  background-color: var(--color-bg-sunken);
}

.comment-item__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-2);
}

.comment-item__author {
  font-weight: 600;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
}

.comment-item__time {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.comment-item__reference {
  margin: 0 0 var(--space-3) 0;
  padding: var(--space-2) var(--space-4);
  border-left: 3px solid var(--color-accent);
  background: var(--color-bg-sunken);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.comment-item__content {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}

/* --- 评论为空 --- */
.comments-empty {
  padding: var(--space-8) 0;
  text-align: center;
}

.comments-empty__text {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

/* --- 订阅表单弹窗 --- */
.subscribe-overlay {
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

.subscribe-form {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 420px;
  box-shadow: var(--shadow-lg);
  animation: formSlideIn var(--transition-base) ease-out;
}

@keyframes formSlideIn {
  from {
    opacity: 0;
    transform: translateY(16px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
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

.subscribe-form__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.subscribe-form__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-6);
  line-height: 1.5;
}

.subscribe-form__user-email {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  margin: 0;
}

.subscribe-form__fields {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

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

.subscribe-form__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

/* --- 版本历史面板 --- */
.revision-overlay {
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

.revision-panel {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 520px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

.revision-panel__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-6);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.revision-panel__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.revision-panel__icon {
  font-size: var(--text-2xl);
  color: var(--color-accent);
}

.revision-panel__close {
  background: none;
  border: none;
  font-size: var(--text-2xl);
  color: var(--color-text-tertiary);
  cursor: pointer;
  padding: 0;
  line-height: 1;
  transition: color var(--transition-fast);
}

.revision-panel__close:hover {
  color: var(--color-text-primary);
}

.revision-panel__loading,
.revision-panel__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-12) var(--space-6);
  gap: var(--space-3);
}

.revision-panel__empty-icon {
  font-size: var(--text-4xl);
  color: var(--color-border);
}

.revision-panel__empty p {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

/* --- 修订版本列表 --- */
.revision-list {
  overflow-y: auto;
  padding: var(--space-4);
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.revision-item {
  padding: var(--space-4);
  background: var(--color-bg);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.revision-item:hover {
  border-color: var(--color-accent-light);
  box-shadow: var(--shadow-sm);
}

.revision-item--selected {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 1px var(--color-accent);
}

.revision-item__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-2);
}

.revision-item__version {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-accent);
  background: var(--color-accent-light);
  padding: 1px var(--space-2);
  border-radius: var(--radius-sm);
}

.revision-item__time {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin-left: auto;
}

.revision-item__title {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-3);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.revision-item__actions {
  display: flex;
  gap: var(--space-2);
}

/* --- Diff 视图模态框 --- */
.diff-modal {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  width: 95%;
  max-width: 800px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

.diff-modal__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.diff-modal__title {
  font-family: var(--font-display);
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.diff-modal__range {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: 400;
  color: var(--color-text-tertiary);
}

.diff-modal__close {
  background: none;
  border: none;
  font-size: var(--text-2xl);
  color: var(--color-text-tertiary);
  cursor: pointer;
  padding: 0;
  line-height: 1;
  transition: color var(--transition-fast);
}

.diff-modal__close:hover {
  color: var(--color-text-primary);
}

.diff-modal__loading,
.diff-modal__error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-12) var(--space-6);
  gap: var(--space-3);
}

.diff-modal__error p {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

/* --- Diff 内容 --- */
.diff-content {
  overflow-y: auto;
  flex: 1;
}

.diff-content__stats {
  display: flex;
  gap: var(--space-4);
  padding: var(--space-3) var(--space-6);
  border-bottom: 1px solid var(--color-border-light);
  background: var(--color-bg-sunken);
}

.diff-stat {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: 600;
}

.diff-stat--add {
  color: var(--color-success);
}

.diff-stat--delete {
  color: var(--color-error);
}

.diff-content__lines {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  line-height: 1.7;
}

.diff-line {
  display: flex;
  padding: 0 var(--space-4);
  border-bottom: 1px solid transparent;
}

.diff-line--add {
  background: rgba(74, 124, 89, 0.08);
  border-left: 3px solid var(--color-success);
}

.diff-line--delete {
  background: rgba(196, 62, 62, 0.08);
  border-left: 3px solid var(--color-error);
}

.diff-line__old-num,
.diff-line__new-num {
  width: var(--space-10);
  flex-shrink: 0;
  text-align: right;
  padding-right: var(--space-2);
  color: var(--color-text-tertiary);
  user-select: none;
}

.diff-line__marker {
  width: var(--space-4);
  flex-shrink: 0;
  text-align: center;
  font-weight: 600;
  user-select: none;
}

.diff-line--add .diff-line__marker {
  color: var(--color-success);
}

.diff-line--delete .diff-line__marker {
  color: var(--color-error);
}

.diff-line__content {
  flex: 1;
  padding-left: var(--space-2);
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--color-text-primary);
}

.diff-line--add .diff-line__content {
  color: var(--color-success);
}

.diff-line--delete .diff-line__content {
  color: var(--color-error);
}

/* --- 回滚确认对话框 --- */
.rollback-dialog {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 400px;
  box-shadow: var(--shadow-lg);
  animation: modalSlideIn var(--transition-base) ease-out;
}

.rollback-dialog__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-3);
}

.rollback-dialog__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin-bottom: var(--space-2);
}

.rollback-dialog__desc strong {
  color: var(--color-accent);
  font-family: var(--font-mono);
}

.rollback-dialog__warning {
  font-size: var(--text-xs);
  color: var(--color-warning);
  line-height: 1.5;
  margin-bottom: var(--space-6);
  padding: var(--space-3);
  background: rgba(196, 155, 62, 0.08);
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--color-warning);
}

.rollback-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

/* --- 响应式 --- */
@media (max-width: 767px) {
  .post-detail {
    max-width: 100%;
  }

  .post-header__title {
    font-size: var(--text-3xl);
  }

  .post-header__excerpt {
    font-size: var(--text-base);
  }

  .subscribe-form {
    padding: var(--space-6);
    margin: var(--space-4);
  }

  .comment-form {
    padding: var(--space-4);
  }

  .revision-panel {
    max-width: 100%;
    max-height: 90vh;
    margin: var(--space-4);
  }

  .diff-modal {
    width: 100%;
    max-width: 100%;
    max-height: 95vh;
    margin: var(--space-2);
    border-radius: var(--radius-md);
  }

  .rollback-dialog {
    padding: var(--space-6);
    margin: var(--space-4);
  }

  .todo-item {
    flex-wrap: wrap;
  }

  .todo-item__inline-subscribe {
    width: 100%;
    margin-top: var(--space-2);
  }

  .todo-item__email-input {
    flex: 1;
    min-width: 0;
  }
}
</style>
