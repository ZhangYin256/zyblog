<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import {
  NSpin,
  NEmpty,
  NButton,
  NPagination,
  NPopconfirm,
  useMessage,
} from 'naive-ui'
import { usePosts } from '../composables/usePosts'

const router = useRouter()
const message = useMessage()
const {
  posts,
  loading,
  error,
  pagination,
  fetchDrafts,
  publishPost,
  deletePost,
  formatDate,
} = usePosts()

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

function editDraft(id: number) {
  router.push(`/publish?edit=${id}`)
}

async function handlePublish(id: number) {
  try {
    await publishPost(id)
    message.success('文章已发布')
    await fetchDrafts(pagination.page, pagination.perPage)
  } catch {
    message.error('发布失败')
  }
}

async function handleDelete(id: number) {
  try {
    await deletePost(id)
    message.success('草稿已删除')
    await fetchDrafts(pagination.page, pagination.perPage)
  } catch {
    message.error('删除失败')
  }
}

function handlePageChange(page: number) {
  fetchDrafts(page, pagination.perPage)
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

onMounted(() => {
  fetchDrafts(1, 10)
})
</script>

<template>
  <div class="drafts">
    <header class="drafts__header">
      <h1 class="drafts__title">草稿箱</h1>
      <p class="drafts__subtitle">尚未发布的文章</p>
    </header>

    <!-- Loading -->
    <div v-if="loading && posts.length === 0" class="drafts__loading">
      <n-spin size="large" />
    </div>

    <!-- Error -->
    <div v-else-if="error" class="drafts__error">
      <p>{{ error }}</p>
      <n-button @click="fetchDrafts(1, 10)">重试</n-button>
    </div>

    <!-- Empty -->
    <div v-else-if="posts.length === 0" class="drafts__empty">
      <n-empty description="没有草稿" />
    </div>

    <!-- Draft List -->
    <div v-else class="draft-list">
      <div v-for="post in posts" :key="post.id" class="draft-card">
        <div class="draft-card__body">
          <div class="draft-card__meta">
            <time class="draft-card__date" :datetime="post.created_at">
              {{ relativeTime(post.created_at) }}
            </time>
          </div>

          <h2 class="draft-card__title">{{ post.title }}</h2>

          <p v-if="post.excerpt" class="draft-card__excerpt">
            {{ truncateExcerpt(post.excerpt) }}
          </p>
        </div>

        <div class="draft-card__actions">
          <n-button
            type="primary"
            size="small"
            @click.stop="handlePublish(post.id)"
            :loading="loading"
          >
            发布
          </n-button>
          <n-button size="small" @click.stop="editDraft(post.id)">
            编辑
          </n-button>
          <n-popconfirm @positive-click="handleDelete(post.id)">
            <template #trigger>
              <n-button type="error" size="small" quaternary @click.stop>
                删除
              </n-button>
            </template>
            确定删除这篇草稿？
          </n-popconfirm>
        </div>
      </div>
    </div>

    <!-- Pagination -->
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
.drafts {
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

.drafts__header {
  margin-bottom: var(--space-10);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.drafts__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.drafts__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

.drafts__loading,
.drafts__error,
.drafts__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.drafts__error p {
  color: var(--color-error);
  font-size: var(--text-base);
}

.draft-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.draft-card {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.draft-card:hover {
  background-color: var(--color-bg-sunken);
}

.draft-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.draft-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.draft-card__date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.draft-card__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.4;
  letter-spacing: -0.01em;
}

.draft-card__excerpt {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.draft-card__actions {
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
  .drafts__title {
    font-size: var(--text-3xl);
  }

  .draft-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .draft-card__actions {
    width: 100%;
  }
}
</style>
