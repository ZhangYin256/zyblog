<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NSpin, NButton, NInput, useMessage } from 'naive-ui'
import { usePosts } from '../composables/usePosts'

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
  createSubscriber,
  formatDate,
} = usePosts()

// Subscriber form state
const subscriberEmail = ref('')
const subscriberName = ref('')
const subscribing = ref(false)
const showSubscribeForm = ref(false)
const activeTodoTitle = ref('')

// Post ID from route
const postId = computed(() => route.params.id as string)

// Format full date
function formatFullDate(isoDate: string): string {
  const date = new Date(isoDate)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long',
  })
}

/**
 * Process content to highlight #todo tags.
 * Replaces #todo patterns with styled spans and optional subscribe button.
 */
function processContent(content: string): string {
  if (!content) return ''

  // Escape HTML to prevent XSS
  let processed = content
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')

  // Convert markdown-like line breaks
  processed = processed.replace(/\n\n/g, '</p><p>')
  processed = processed.replace(/\n/g, '<br>')

  // Highlight #todo tags
  processed = processed.replace(
    /#todo\b([^#]*?)(?=#todo|$)/g,
    (match, taskText) => {
      const task = taskText.trim() || '待办事项'
      const escapedTask = task.replace(/"/g, '&quot;')
      return `<span class="todo-highlight" data-task="${escapedTask}">
        <span class="todo-highlight__tag">#todo</span>
        <span class="todo-highlight__text">${task}</span>
        <button class="todo-highlight__subscribe" onclick="window.__handleTodoSubscribe('${escapedTask}')">
          订阅更新
        </button>
      </span>`
    }
  )

  // Wrap in paragraphs
  if (!processed.startsWith('<p>')) {
    processed = '<p>' + processed + '</p>'
  }

  return processed
}

// Handle todo subscribe click (exposed to window for inline onclick)
async function handleTodoSubscribe(taskTitle: string) {
  activeTodoTitle.value = taskTitle
  showSubscribeForm.value = true
}

// Expose to window for inline onclick handlers
if (typeof window !== 'undefined') {
  (window as any).__handleTodoSubscribe = handleTodoSubscribe
}

// Submit subscriber
async function handleSubscribe() {
  if (!subscriberEmail.value.trim()) {
    message.warning('请输入邮箱地址')
    return
  }

  subscribing.value = true
  try {
    const result = await createSubscriber({
      email: subscriberEmail.value.trim(),
      name: subscriberName.value.trim() || undefined,
    })

    if (result.success) {
      message.success('订阅成功！当待办事项更新时会通知您。')
      showSubscribeForm.value = false
      subscriberEmail.value = ''
      subscriberName.value = ''
    } else {
      message.error(result.error || '订阅失败，请重试')
    }
  } catch {
    message.error('订阅失败，请重试')
  } finally {
    subscribing.value = false
  }
}

// Cancel subscribe
function cancelSubscribe() {
  showSubscribeForm.value = false
  subscriberEmail.value = ''
  subscriberName.value = ''
}

// Navigate back
function goBack() {
  router.push('/')
}

onMounted(async () => {
  await fetchPost(postId.value)
  await fetchPostTodos(postId.value)

  // Re-attach event listeners after content renders
  await nextTick()
  const subscribeButtons = document.querySelectorAll('.todo-highlight__subscribe')
  subscribeButtons.forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.preventDefault()
      const task = (btn as HTMLElement).closest('.todo-highlight')?.getAttribute('data-task') || ''
      handleTodoSubscribe(task)
    })
  })
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
        </div>

        <h1 class="post-header__title">{{ currentPost.title }}</h1>

        <p v-if="currentPost.excerpt" class="post-header__excerpt">
          {{ currentPost.excerpt }}
        </p>
      </header>

      <!-- Cover Image -->
      <div v-if="currentPost.cover_image" class="post-cover">
        <img :src="currentPost.cover_image" :alt="currentPost.title" />
      </div>

      <!-- Post Body -->
      <div
        class="post-body"
        v-html="processContent(currentPost.content)"
      />

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
            <button
              class="todo-item__subscribe"
              @click="handleTodoSubscribe(todo.title)"
            >
              订阅更新
            </button>
          </li>
        </ul>
      </section>

      <!-- Subscribe Modal/Form -->
      <div v-if="showSubscribeForm" class="subscribe-overlay" @click.self="cancelSubscribe">
        <div class="subscribe-form">
          <h3 class="subscribe-form__title">订阅待办更新</h3>
          <p class="subscribe-form__desc">
            当「{{ activeTodoTitle }}」有更新时，我们会通过邮件通知您。
          </p>

          <div class="subscribe-form__fields">
            <div class="form-field">
              <label class="form-field__label" for="sub-email">邮箱地址 *</label>
              <n-input
                id="sub-email"
                v-model:value="subscriberEmail"
                type="text"
                placeholder="your@email.com"
                :disabled="subscribing"
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
    </article>
  </div>
</template>

<style scoped>
/* --- Post Detail Layout --- */
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

/* --- Back Button --- */
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

/* --- Loading / Error States --- */
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

/* --- Post Header --- */
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

/* --- Cover Image --- */
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

/* --- Post Body (rendered HTML) --- */
.post-body {
  font-family: var(--font-body);
  font-size: var(--text-base);
  line-height: 1.8;
  color: var(--color-text-primary);
}

.post-body :deep(p) {
  margin-bottom: var(--space-4);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  line-height: 1.8;
}

.post-body :deep(h2) {
  font-family: var(--font-display);
  font-size: var(--text-2xl);
  font-weight: 600;
  margin-top: var(--space-8);
  margin-bottom: var(--space-4);
  color: var(--color-text-primary);
}

.post-body :deep(h3) {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  margin-top: var(--space-6);
  margin-bottom: var(--space-3);
  color: var(--color-text-primary);
}

.post-body :deep(code) {
  font-family: var(--font-mono);
  font-size: 0.9em;
  background: var(--color-bg-sunken);
  padding: 2px var(--space-1);
  border-radius: var(--radius-sm);
  color: var(--color-accent);
}

.post-body :deep(pre) {
  background: var(--color-bg-sidebar);
  color: var(--color-text-inverse);
  padding: var(--space-4);
  border-radius: var(--radius-md);
  overflow-x: auto;
  margin-bottom: var(--space-4);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: 1.6;
}

.post-body :deep(pre code) {
  background: none;
  padding: 0;
  color: inherit;
}

.post-body :deep(blockquote) {
  border-left: 3px solid var(--color-accent);
  padding-left: var(--space-4);
  margin: var(--space-4) 0;
  color: var(--color-text-secondary);
  font-style: italic;
}

.post-body :deep(a) {
  color: var(--color-accent);
  text-decoration: underline;
  text-underline-offset: 2px;
  transition: color var(--transition-fast);
}

.post-body :deep(a:hover) {
  color: var(--color-accent-hover);
}

.post-body :deep(ul),
.post-body :deep(ol) {
  margin-bottom: var(--space-4);
  padding-left: var(--space-6);
}

.post-body :deep(li) {
  margin-bottom: var(--space-2);
}

.post-body :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.post-body :deep(em) {
  font-style: italic;
}

/* --- #todo Highlight Styles --- */
.post-body :deep(.todo-highlight) {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  background: linear-gradient(135deg, rgba(196, 93, 62, 0.08), rgba(196, 93, 62, 0.15));
  border: 1px solid rgba(196, 93, 62, 0.2);
  border-radius: var(--radius-md);
  padding: var(--space-1) var(--space-3);
  margin: 0 var(--space-1);
  font-size: var(--text-sm);
  vertical-align: baseline;
  transition: all var(--transition-fast);
}

.post-body :deep(.todo-highlight:hover) {
  background: linear-gradient(135deg, rgba(196, 93, 62, 0.12), rgba(196, 93, 62, 0.2));
  border-color: rgba(196, 93, 62, 0.3);
  box-shadow: var(--shadow-sm);
}

.post-body :deep(.todo-highlight__tag) {
  font-weight: 600;
  color: var(--color-accent);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-body :deep(.todo-highlight__text) {
  color: var(--color-text-primary);
  font-weight: 500;
}

.post-body :deep(.todo-highlight__subscribe) {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px var(--space-2);
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
  opacity: 0;
  transform: scale(0.95);
}

.post-body :deep(.todo-highlight:hover .todo-highlight__subscribe) {
  opacity: 1;
  transform: scale(1);
}

.post-body :deep(.todo-highlight__subscribe:hover) {
  background: var(--color-accent-hover);
  transform: scale(1.05);
}

.post-body :deep(.todo-highlight__subscribe:active) {
  transform: scale(0.98);
}

/* --- Todo Section --- */
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

/* --- Subscribe Form Overlay --- */
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

/* --- Responsive --- */
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

  .post-body :deep(.todo-highlight) {
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .post-body :deep(.todo-highlight__subscribe) {
    opacity: 1;
    transform: scale(1);
    width: 100%;
    justify-content: center;
    margin-top: var(--space-1);
  }

  .subscribe-form {
    padding: var(--space-6);
    margin: var(--space-4);
  }
}
</style>
