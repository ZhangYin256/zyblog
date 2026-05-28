<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  NSpin,
  NEmpty,
  NTag,
} from 'naive-ui'
import { useAuth } from '../composables/useAuth'
import type { Comment } from '../composables/useComments'
import api from '../lib/api'

const router = useRouter()
const { user } = useAuth()

const comments = ref<Comment[]>([])
const loading = ref(false)
const postTitles = ref<Record<number, string>>({})

function formatDate(isoDate: string): string {
  return new Date(isoDate).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function truncateContent(content: string, maxLen = 120): string {
  if (content.length <= maxLen) return content
  return content.slice(0, maxLen) + '...'
}

// Filter state
type FilterMode = 'all' | 'approved' | 'pending'
const filterMode = ref<FilterMode>('all')

const filteredComments = computed(() => {
  if (filterMode.value === 'all') return comments.value
  if (filterMode.value === 'approved') return comments.value.filter((c) => c.approved)
  return comments.value.filter((c) => !c.approved)
})

const statusCounts = computed(() => ({
  all: comments.value.length,
  approved: comments.value.filter((c) => c.approved).length,
  pending: comments.value.filter((c) => !c.approved).length,
}))

async function fetchMyComments() {
  if (!user.value) return

  loading.value = true
  try {
    // Fetch all posts
    const { data: postsData } = await api.get('/api/v1/posts', { params: { per_page: 1000 } })
    const posts = postsData.items ?? []

    // Build title map
    const titleMap: Record<number, string> = {}
    for (const post of posts) {
      titleMap[post.id] = post.title
    }
    postTitles.value = titleMap

    // Fetch approved comments for each post and filter by user
    const allComments: Comment[] = []
    const userName = user.value.name
    const userEmail = user.value.email

    await Promise.all(
      posts.map(async (post: { id: number }) => {
        try {
          const { data } = await api.get<Comment[]>(`/api/v1/posts/${post.id}/comments`)
          const myComments = data.filter(
            (c) => c.author_name === userName || c.author_email === userEmail
          )
          allComments.push(...myComments)
        } catch {
          // Skip posts that fail
        }
      })
    )

    // Sort by created_at desc
    allComments.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    comments.value = allComments
  } catch (err) {
    console.error('Failed to fetch comments:', err)
  } finally {
    loading.value = false
  }
}

function goToPost(postId: number) {
  router.push(`/posts/${postId}`)
}

onMounted(() => {
  fetchMyComments()
})
</script>

<template>
  <div class="my-comments">
    <header class="my-comments__header">
      <h1 class="my-comments__title">我的评论</h1>
      <p class="my-comments__subtitle">查看你发表的所有评论</p>
    </header>

    <!-- Filter Tabs -->
    <div class="my-comments__filters">
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'all' }"
        @click="filterMode = 'all'"
      >
        全部
        <span class="filter-tab__count">{{ statusCounts.all }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'approved' }"
        @click="filterMode = 'approved'"
      >
        已通过
        <span class="filter-tab__count">{{ statusCounts.approved }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'pending' }"
        @click="filterMode = 'pending'"
      >
        待审核
        <span class="filter-tab__count">{{ statusCounts.pending }}</span>
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading && comments.length === 0" class="my-comments__loading">
      <n-spin size="large" />
    </div>

    <!-- Empty -->
    <div v-else-if="filteredComments.length === 0" class="my-comments__empty">
      <n-empty :description="filterMode === 'all' ? '暂无评论' : '该状态下暂无评论'" />
    </div>

    <!-- Comment List -->
    <div v-else class="comment-list">
      <div
        v-for="comment in filteredComments"
        :key="comment.id"
        class="comment-card"
        @click="goToPost(comment.post_id)"
      >
        <div class="comment-card__body">
          <div class="comment-card__header">
            <n-tag
              :type="comment.approved ? 'success' : 'warning'"
              :bordered="false"
              size="small"
            >
              {{ comment.approved ? '已通过' : '待审核' }}
            </n-tag>
            <span class="comment-card__id">#{{ comment.id }}</span>
          </div>

          <p class="comment-card__post">
            文章: {{ postTitles[comment.post_id] ?? `#${comment.post_id}` }}
          </p>

          <p class="comment-card__content">{{ truncateContent(comment.content) }}</p>

          <div class="comment-card__meta">
            <time class="comment-card__date" :datetime="comment.created_at">
              {{ formatDate(comment.created_at) }}
            </time>
          </div>
        </div>

        <div class="comment-card__arrow">→</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.my-comments {
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

/* --- Header --- */
.my-comments__header {
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.my-comments__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.my-comments__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Filter Tabs --- */
.my-comments__filters {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-8);
}

.filter-tab {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: var(--font-body);
}

.filter-tab:hover {
  background-color: var(--color-bg-sunken);
  color: var(--color-text-primary);
}

.filter-tab--active {
  background-color: var(--color-accent);
  color: var(--color-text-inverse);
  border-color: var(--color-accent);
}

.filter-tab--active:hover {
  background-color: var(--color-accent-hover);
  color: var(--color-text-inverse);
}

.filter-tab__count {
  font-size: var(--text-xs);
  font-variant-numeric: tabular-nums;
  opacity: 0.8;
}

/* --- Loading / Empty --- */
.my-comments__loading,
.my-comments__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

/* --- Comment List --- */
.comment-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.comment-card {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
  cursor: pointer;
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

.comment-card__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.comment-card__id {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.comment-card__post {
  font-size: var(--text-sm);
  color: var(--color-accent);
  font-weight: 500;
}

.comment-card__content {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}

.comment-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.comment-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.comment-card__arrow {
  flex-shrink: 0;
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  transition: color var(--transition-fast), transform var(--transition-fast);
}

.comment-card:hover .comment-card__arrow {
  color: var(--color-accent);
  transform: translateX(2px);
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .my-comments__title {
    font-size: var(--text-3xl);
  }

  .my-comments__filters {
    flex-wrap: wrap;
  }

  .comment-card {
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .comment-card__arrow {
    display: none;
  }
}
</style>
