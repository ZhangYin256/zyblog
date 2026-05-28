<script setup lang="ts">
import { onMounted, computed } from 'vue'
import {
  NSpin,
  NEmpty,
  NButton,
  NPagination,
  NPopconfirm,
  useMessage,
} from 'naive-ui'
import { usePosts, type TrashPost } from '../composables/usePosts'

const message = useMessage()
const {
  posts,
  loading,
  error,
  pagination,
  fetchTrashPosts,
  restorePost,
  permanentDeletePost,
  formatDate,
} = usePosts()

const trashPosts = computed(() => posts.value as unknown as TrashPost[])

const totalPages = computed(() =>
  Math.ceil(pagination.total / pagination.perPage)
)

function relativeTime(isoDate: string): string {
  const now = new Date()
  const date = new Date(isoDate)
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) return '今天'
  if (diffDays === 1) return '昨天'
  if (diffDays < 7) return `${diffDays} 天前`
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} 周前`
  return formatDate(isoDate)
}

function truncateExcerpt(text: string | null, maxLen = 100): string {
  if (!text) return ''
  if (text.length <= maxLen) return text
  return text.slice(0, maxLen).trimEnd() + '…'
}

async function handleRestore(id: number) {
  try {
    await restorePost(id)
    message.success('文章已恢复')
    await fetchTrashPosts(pagination.page, pagination.perPage)
  } catch {
    message.error('恢复失败')
  }
}

async function handlePermanentDelete(id: number) {
  try {
    await permanentDeletePost(id)
    message.success('文章已永久删除')
    await fetchTrashPosts(pagination.page, pagination.perPage)
  } catch {
    message.error('删除失败')
  }
}

function handlePageChange(page: number) {
  fetchTrashPosts(page, pagination.perPage)
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

onMounted(() => {
  fetchTrashPosts(1, 10)
})
</script>

<template>
  <div class="trash">
    <header class="trash__header">
      <h1 class="trash__title">回收站</h1>
      <p class="trash__subtitle">已删除的文章，可恢复或永久删除</p>
    </header>

    <div v-if="loading && trashPosts.length === 0" class="trash__loading">
      <n-spin size="large" />
    </div>

    <div v-else-if="error" class="trash__error">
      <p>{{ error }}</p>
      <n-button @click="fetchTrashPosts(1, 10)">重试</n-button>
    </div>

    <div v-else-if="trashPosts.length === 0" class="trash__empty">
      <n-empty description="回收站是空的" />
    </div>

    <div v-else class="trash-list">
      <div v-for="post in trashPosts" :key="post.id" class="trash-card">
        <div class="trash-card__body">
          <div class="trash-card__meta">
            <time class="trash-card__date" :datetime="post.deleted_at">
              删除于 {{ relativeTime(post.deleted_at) }}
            </time>
          </div>

          <h2 class="trash-card__title">{{ post.title }}</h2>

          <p v-if="post.excerpt" class="trash-card__excerpt">
            {{ truncateExcerpt(post.excerpt) }}
          </p>
        </div>

        <div class="trash-card__actions">
          <n-button
            type="primary"
            size="small"
            @click.stop="handleRestore(post.id)"
            :loading="loading"
          >
            恢复
          </n-button>
          <n-popconfirm @positive-click="handlePermanentDelete(post.id)">
            <template #trigger>
              <n-button type="error" size="small" quaternary @click.stop>
                永久删除
              </n-button>
            </template>
            永久删除后无法恢复，确定继续？
          </n-popconfirm>
        </div>
      </div>
    </div>

    <div v-if="totalPages > 1" class="pagination-wrapper">
      <n-pagination
        v-model:page="pagination.page"
        :page-count="totalPages"
        :page-slot="5"
        @update:page="handlePageChange"
      />
    </div>
  </div>
</template>

<style scoped>
.trash {
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

.trash__header {
  margin-bottom: var(--space-10);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.trash__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.trash__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

.trash__loading,
.trash__error,
.trash__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.trash__error p {
  color: var(--color-error);
  font-size: var(--text-base);
}

.trash-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.trash-card {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.trash-card:hover {
  background-color: var(--color-bg-sunken);
}

.trash-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.trash-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.trash-card__date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.trash-card__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.4;
  letter-spacing: -0.01em;
}

.trash-card__excerpt {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.trash-card__actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border-light);
}

@media (max-width: 767px) {
  .trash__title {
    font-size: var(--text-3xl);
  }

  .trash-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .trash-card__actions {
    width: 100%;
  }
}
</style>
