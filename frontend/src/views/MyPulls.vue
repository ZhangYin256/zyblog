<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  NSpin,
  NEmpty,
  NTag,
} from 'naive-ui'
import { useAuth } from '../composables/useAuth'
import type { PullRequest } from '../composables/usePullRequests'
import api from '../lib/api'

const router = useRouter()
const { user } = useAuth()

const pulls = ref<PullRequest[]>([])
const loading = ref(false)
const postTitles = ref<Record<number, string>>({})

// PR status helpers
function getStatusLabel(status: string): string {
  const map: Record<string, string> = {
    open: '开放',
    closed: '已关闭',
    merged: '已合并',
  }
  return map[status] ?? status
}

function getStatusTagType(status: string): 'success' | 'error' | 'info' | 'default' {
  const map: Record<string, 'success' | 'error' | 'info' | 'default'> = {
    open: 'success',
    closed: 'error',
    merged: 'info',
  }
  return map[status] ?? 'default'
}

function formatDate(isoDate: string): string {
  return new Date(isoDate).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

// Filter state
type FilterMode = 'all' | 'open' | 'merged' | 'closed'
const filterMode = ref<FilterMode>('all')

const filteredPulls = computed(() => {
  if (filterMode.value === 'all') return pulls.value
  return pulls.value.filter((p) => p.status === filterMode.value)
})

const statusCounts = computed(() => ({
  all: pulls.value.length,
  open: pulls.value.filter((p) => p.status === 'open').length,
  merged: pulls.value.filter((p) => p.status === 'merged').length,
  closed: pulls.value.filter((p) => p.status === 'closed').length,
}))

async function fetchMyPulls() {
  if (!user.value?.email) return

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

    // Fetch PRs for each post and filter by user email
    const allPulls: PullRequest[] = []
    const userEmail = user.value.email

    await Promise.all(
      posts.map(async (post: { id: number }) => {
        try {
          const { data } = await api.get(`/api/v1/posts/${post.id}/pulls`)
          const postPulls: PullRequest[] = data.items ?? []
          const myPulls = postPulls.filter((p) => p.user_email === userEmail)
          allPulls.push(...myPulls)
        } catch {
          // Skip posts that fail
        }
      })
    )

    // Sort by created_at desc
    allPulls.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    pulls.value = allPulls
  } catch (err) {
    console.error('Failed to fetch pulls:', err)
  } finally {
    loading.value = false
  }
}

function goToPost(postId: number) {
  router.push(`/posts/${postId}`)
}

onMounted(() => {
  fetchMyPulls()
})
</script>

<template>
  <div class="my-pulls">
    <header class="my-pulls__header">
      <h1 class="my-pulls__title">我的 PR</h1>
      <p class="my-pulls__subtitle">查看你提交的所有拉取请求</p>
    </header>

    <!-- Filter Tabs -->
    <div class="my-pulls__filters">
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
        :class="{ 'filter-tab--active': filterMode === 'open' }"
        @click="filterMode = 'open'"
      >
        开放
        <span class="filter-tab__count">{{ statusCounts.open }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'merged' }"
        @click="filterMode = 'merged'"
      >
        已合并
        <span class="filter-tab__count">{{ statusCounts.merged }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'closed' }"
        @click="filterMode = 'closed'"
      >
        已关闭
        <span class="filter-tab__count">{{ statusCounts.closed }}</span>
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading && pulls.length === 0" class="my-pulls__loading">
      <n-spin size="large" />
    </div>

    <!-- Empty -->
    <div v-else-if="filteredPulls.length === 0" class="my-pulls__empty">
      <n-empty :description="filterMode === 'all' ? '暂无提交的 PR' : '该状态下暂无 PR'" />
    </div>

    <!-- Pull List -->
    <div v-else class="pull-list">
      <div
        v-for="pull in filteredPulls"
        :key="pull.id"
        class="pull-card"
        @click="goToPost(pull.post_id)"
      >
        <div class="pull-card__body">
          <div class="pull-card__header">
            <n-tag
              :type="getStatusTagType(pull.status)"
              :bordered="false"
              size="small"
            >
              {{ getStatusLabel(pull.status) }}
            </n-tag>
            <span class="pull-card__id">#{{ pull.id }}</span>
          </div>

          <p class="pull-card__post">
            文章: {{ postTitles[pull.post_id] ?? `#${pull.post_id}` }}
          </p>

          <p v-if="pull.message" class="pull-card__message">{{ pull.message }}</p>

          <div class="pull-card__meta">
            <span class="pull-card__email">{{ pull.user_email }}</span>
            <time class="pull-card__date" :datetime="pull.created_at">
              {{ formatDate(pull.created_at) }}
            </time>
          </div>
        </div>

        <div class="pull-card__arrow">→</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.my-pulls {
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
.my-pulls__header {
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.my-pulls__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.my-pulls__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Filter Tabs --- */
.my-pulls__filters {
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
.my-pulls__loading,
.my-pulls__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

/* --- Pull List --- */
.pull-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.pull-card {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  padding: var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
  cursor: pointer;
}

.pull-card:hover {
  background-color: var(--color-bg-sunken);
}

.pull-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.pull-card__header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.pull-card__id {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.pull-card__post {
  font-size: var(--text-sm);
  color: var(--color-accent);
  font-weight: 500;
}

.pull-card__message {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.pull-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
}

.pull-card__email {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.pull-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.pull-card__arrow {
  flex-shrink: 0;
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  transition: color var(--transition-fast), transform var(--transition-fast);
}

.pull-card:hover .pull-card__arrow {
  color: var(--color-accent);
  transform: translateX(2px);
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .my-pulls__title {
    font-size: var(--text-3xl);
  }

  .my-pulls__filters {
    flex-wrap: wrap;
  }

  .pull-card {
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .pull-card__arrow {
    display: none;
  }
}
</style>
