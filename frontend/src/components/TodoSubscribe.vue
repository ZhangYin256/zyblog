<script setup lang="ts">
import { ref, computed } from 'vue'
import { NInput, NButton, NTag, NDivider } from 'naive-ui'
import { useMessage } from 'naive-ui'
import api from '../lib/api'

interface Props {
  /** 文章中的 #todo 标签字符串列表 */
  tags?: string[]
  /** 可选的文章 ID，用于跟踪触发订阅的文章 */
  postId?: number
  /** 可选：显示当前订阅者数量 */
  subscriberCount?: number
}

const props = withDefaults(defineProps<Props>(), {
  tags: () => [],
  postId: undefined,
  subscriberCount: undefined,
})

const message = useMessage()

// 表单状态
const email = ref('')
const isSubmitting = ref(false)
const isSubscribed = ref(false)

// 邮箱验证
const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
const isValidEmail = computed(() => emailRegex.test(email.value))
const showEmailError = ref(false)

// 如果标签没有 # 前缀则格式化添加
const formattedTags = computed(() =>
  props.tags.map((tag) => (tag.startsWith('#') ? tag : `#${tag}`))
)

// 订阅者数量显示
const displayCount = computed(() => {
  if (props.subscriberCount === undefined) return null
  if (props.subscriberCount >= 1000) {
    return `${(props.subscriberCount / 1000).toFixed(1)}k`
  }
  return props.subscriberCount.toString()
})

async function handleSubscribe() {
  // 重置错误状态
  showEmailError.value = false

  // 验证邮箱
  if (!email.value.trim()) {
    showEmailError.value = true
    message.warning('请输入邮箱地址')
    return
  }

  if (!isValidEmail.value) {
    showEmailError.value = true
    message.error('邮箱格式错误')
    return
  }

  isSubmitting.value = true

  try {
    await api.post('/api/v1/subscribers', {
      email: email.value.trim(),
      post_id: props.postId,
    })

    isSubscribed.value = true
    message.success('订阅成功')
    email.value = ''
  } catch (error: any) {
    if (error.response) {
      const status = error.response.status
      const errorMsg = error.response.data?.message || error.response.data?.error

      if (status === 409 || errorMsg?.includes('already') || errorMsg?.includes('duplicate')) {
        message.warning('已订阅')
      } else if (status === 422) {
        message.error('邮箱格式错误')
      } else {
        message.error(errorMsg || '订阅失败，请稍后重试')
      }
    } else {
      message.error('订阅失败，请稍后重试')
    }
  } finally {
    isSubmitting.value = false
  }
}

function handleEmailInput() {
  if (showEmailError.value && isValidEmail.value) {
    showEmailError.value = false
  }
}
</script>

<template>
  <div class="todo-subscribe">
    <!-- Header with decorative accent -->
    <div class="todo-subscribe__header">
      <span class="todo-subscribe__icon">✦</span>
      <h4 class="todo-subscribe__title">待办事项订阅</h4>
    </div>

    <!-- Tag list -->
    <div v-if="formattedTags.length > 0" class="todo-subscribe__tags">
      <span class="todo-subscribe__tags-label">相关标签</span>
      <div class="todo-subscribe__tags-list">
        <n-tag
          v-for="tag in formattedTags"
          :key="tag"
          :bordered="false"
          size="medium"
          class="todo-subscribe__tag"
        >
          {{ tag }}
        </n-tag>
      </div>
    </div>

    <n-divider class="todo-subscribe__divider" />

    <!-- Subscription form -->
    <div v-if="!isSubscribed" class="todo-subscribe__form">
      <p class="todo-subscribe__description">
        订阅此待办事项，获取最新进展通知
      </p>

      <div class="todo-subscribe__input-group">
        <n-input
          v-model:value="email"
          placeholder="your@email.com"
          :status="showEmailError ? 'error' : undefined"
          :disabled="isSubmitting"
          size="large"
          class="todo-subscribe__input"
          @input="handleEmailInput"
          @keyup.enter="handleSubscribe"
        />
        <n-button
          type="primary"
          size="large"
          :loading="isSubmitting"
          :disabled="isSubmitting"
          class="todo-subscribe__button"
          @click="handleSubscribe"
        >
          {{ isSubmitting ? '订阅中...' : '订阅' }}
        </n-button>
      </div>

      <p v-if="showEmailError" class="todo-subscribe__error">
        请输入有效的邮箱地址
      </p>
    </div>

    <!-- Success state -->
    <div v-else class="todo-subscribe__success">
      <div class="todo-subscribe__success-icon">✓</div>
      <p class="todo-subscribe__success-text">订阅成功</p>
      <p class="todo-subscribe__success-hint">
        我们将通过邮件通知您最新进展
      </p>
    </div>

    <!-- Subscriber count (optional) -->
    <div v-if="displayCount !== null" class="todo-subscribe__count">
      <span class="todo-subscribe__count-number">{{ displayCount }}</span>
      <span class="todo-subscribe__count-label">人已订阅</span>
    </div>
  </div>
</template>

<style scoped>
.todo-subscribe {
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
  transition: box-shadow var(--transition-base);
}

.todo-subscribe:hover {
  box-shadow: var(--shadow-md);
}

/* 头部 */
.todo-subscribe__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-5);
}

.todo-subscribe__icon {
  font-size: var(--text-xl);
  color: var(--color-accent);
  line-height: 1;
}

.todo-subscribe__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
  letter-spacing: -0.01em;
}

/* 标签区域 */
.todo-subscribe__tags {
  margin-bottom: var(--space-4);
}

.todo-subscribe__tags-label {
  display: block;
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  margin-bottom: var(--space-2);
}

.todo-subscribe__tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.todo-subscribe__tag {
  background: var(--color-accent-light);
  color: var(--color-accent);
  font-weight: 500;
  font-size: var(--text-sm);
  border: none;
  transition: background-color var(--transition-fast), transform var(--transition-fast);
}

.todo-subscribe__tag:hover {
  background: var(--color-accent);
  color: var(--color-text-inverse);
  transform: translateY(-1px);
}

/* 分割线 */
.todo-subscribe__divider {
  margin: var(--space-4) 0;
  border-color: var(--color-border-light);
}

/* 表单 */
.todo-subscribe__form {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.todo-subscribe__description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.5;
  margin: 0;
}

.todo-subscribe__input-group {
  display: flex;
  gap: var(--space-3);
}

.todo-subscribe__input {
  flex: 1;
}

.todo-subscribe__input :deep(.n-input__border),
.todo-subscribe__input :deep(.n-input__state-border) {
  border-color: var(--color-border);
  transition: border-color var(--transition-fast);
}

.todo-subscribe__input:hover :deep(.n-input__border),
.todo-subscribe__input:hover :deep(.n-input__state-border) {
  border-color: var(--color-accent);
}

.todo-subscribe__button {
  background: var(--color-accent);
  border-color: var(--color-accent);
  font-weight: 600;
  letter-spacing: 0.02em;
  transition: background-color var(--transition-fast), transform var(--transition-fast);
  flex-shrink: 0;
}

.todo-subscribe__button:hover {
  background: var(--color-accent-hover);
  border-color: var(--color-accent-hover);
  transform: translateY(-1px);
}

.todo-subscribe__button:active {
  transform: translateY(0);
}

.todo-subscribe__error {
  font-size: var(--text-xs);
  color: var(--color-error);
  margin: calc(-1 * var(--space-2)) 0 0;
  animation: fadeIn 0.2s ease;
}

/* 成功状态 */
.todo-subscribe__success {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--space-4) 0;
  animation: fadeIn 0.4s ease;
}

.todo-subscribe__success-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-success);
  color: white;
  font-size: var(--text-2xl);
  font-weight: 700;
  border-radius: 50%;
  margin-bottom: var(--space-3);
  animation: scaleIn 0.3s ease;
}

.todo-subscribe__success-text {
  font-family: var(--font-display);
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-success);
  margin: 0 0 var(--space-1);
}

.todo-subscribe__success-hint {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  margin: 0;
}

/* 订阅者数量 */
.todo-subscribe__count {
  display: flex;
  align-items: baseline;
  justify-content: center;
  gap: var(--space-1);
  margin-top: var(--space-4);
  padding-top: var(--space-4);
  border-top: 1px solid var(--color-border-light);
}

.todo-subscribe__count-number {
  font-family: var(--font-display);
  font-size: var(--text-2xl);
  font-weight: 700;
  color: var(--color-accent);
  line-height: 1;
}

.todo-subscribe__count-label {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

/* 动画 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.8);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* 响应式调整 */
@media (max-width: 480px) {
  .todo-subscribe {
    padding: var(--space-4);
  }

  .todo-subscribe__input-group {
    flex-direction: column;
  }

  .todo-subscribe__button {
    width: 100%;
  }
}
</style>
