<script setup lang="ts">
import { onMounted } from 'vue'
import {
  NSpin,
  NEmpty,
  NButton,
  NPopconfirm,
  useMessage,
} from 'naive-ui'
import { useComments } from '../composables/useComments'

const message = useMessage()
const {
  pendingComments,
  loading,
  fetchPendingComments,
  approveComment,
  deleteComment,
  formatDate,
} = useComments()

async function handleApprove(commentId: number) {
  const result = await approveComment(commentId)
  if (result.success) {
    message.success('评论已审核通过')
  } else {
    message.error(result.error || '审核失败')
  }
}

async function handleDelete(commentId: number) {
  const result = await deleteComment(commentId)
  if (result.success) {
    message.success('评论已删除')
  } else {
    message.error(result.error || '删除失败')
  }
}

onMounted(() => {
  fetchPendingComments()
})
</script>

<template>
  <div class="comments-admin">
    <header class="comments-admin__header">
      <h1 class="comments-admin__title">评论管理</h1>
      <p class="comments-admin__subtitle">审核和管理读者评论</p>
    </header>

    <!-- Loading -->
    <div v-if="loading && pendingComments.length === 0" class="comments-admin__loading">
      <n-spin size="large" />
    </div>

    <!-- Empty -->
    <div v-else-if="pendingComments.length === 0" class="comments-admin__empty">
      <n-empty description="没有待审核的评论" />
    </div>

    <!-- Pending Comments List -->
    <div v-else class="comment-list">
      <div
        v-for="comment in pendingComments"
        :key="comment.id"
        class="comment-card"
      >
        <div class="comment-card__body">
          <div class="comment-card__meta">
            <span class="comment-card__author">{{ comment.author_name }}</span>
            <span v-if="comment.author_email" class="comment-card__email">
              {{ comment.author_email }}
            </span>
            <time class="comment-card__date" :datetime="comment.created_at">
              {{ formatDate(comment.created_at) }}
            </time>
          </div>

          <p class="comment-card__post-info">
            文章 ID: {{ comment.post_id }}
          </p>

          <p class="comment-card__content">{{ comment.content }}</p>
        </div>

        <div class="comment-card__actions">
          <n-button
            type="primary"
            size="small"
            @click.stop="handleApprove(comment.id)"
            :loading="loading"
          >
            通过
          </n-button>
          <n-popconfirm @positive-click="handleDelete(comment.id)">
            <template #trigger>
              <n-button type="error" size="small" quaternary @click.stop>
                删除
              </n-button>
            </template>
            确定删除这条评论？
          </n-popconfirm>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.comments-admin {
  animation: fadeIn var(--transition-slow) ease-out;
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

.comments-admin__header {
  margin-bottom: var(--space-10);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.comments-admin__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.comments-admin__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

.comments-admin__loading,
.comments-admin__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.comment-card {
  display: flex;
  align-items: flex-start;
  gap: var(--space-6);
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.comment-card:hover {
  background-color: var(--color-bg-sunken);
}

.comment-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.comment-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
}

.comment-card__author {
  font-weight: 600;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
}

.comment-card__email {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.comment-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.comment-card__post-info {
  font-size: var(--text-xs);
  color: var(--color-accent);
  font-family: var(--font-mono);
}

.comment-card__content {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}

.comment-card__actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

@media (max-width: 767px) {
  .comments-admin__title {
    font-size: var(--text-3xl);
  }

  .comment-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .comment-card__actions {
    width: 100%;
  }
}
</style>
