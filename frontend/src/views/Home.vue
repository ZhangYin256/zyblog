<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { NSpin, NEmpty, NButton } from 'naive-ui'
import { usePosts } from '../composables/usePosts'

const router = useRouter()
const {
  posts,
  loading,
  error,
  pagination,
  fetchPosts,
  formatDate,
} = usePosts()

// Compute total pages
const totalPages = computed(() =>
  Math.ceil(pagination.total / pagination.perPage)
)

// Navigate to post detail
function goToPost(id: number) {
  router.push(`/posts/${id}`)
}

// Handle page change
function handlePageChange(page: number) {
  fetchPosts(page, pagination.perPage)
  // Scroll to top on page change
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// Format relative time (e.g., "3 天前")
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

// Truncate excerpt to a reasonable length
function truncateExcerpt(text: string | null, maxLen = 120): string {
  if (!text) return ''
  if (text.length <= maxLen) return text
  return text.slice(0, maxLen).trimEnd() + '…'
}

onMounted(() => {
  fetchPosts(1, 10, 'published')
})
</script>

<template>
  <div class="home">
    <!-- Page Header -->
    <header class="home__header">
      <h1 class="home__title">文章</h1>
      <p class="home__subtitle">思考、记录与分享</p>
    </header>

    <!-- Loading State -->
    <div v-if="loading && posts.length === 0" class="home__loading">
      <n-spin size="large" />
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="home__error">
      <p>{{ error }}</p>
      <n-button @click="fetchPosts(1, 10, 'published')">重试</n-button>
    </div>

    <!-- Empty State -->
    <div v-else-if="posts.length === 0" class="home__empty">
      <n-empty description="暂无文章" />
    </div>

    <!-- Post List -->
    <div v-else class="post-list">
      <article
        v-for="post in posts"
        :key="post.id"
        class="post-card"
        @click="goToPost(post.id)"
      >
        <!-- Cover Image (if exists) -->
        <div v-if="post.cover_image" class="post-card__cover">
          <img :src="post.cover_image" :alt="post.title" loading="lazy" />
        </div>

        <!-- Content -->
        <div class="post-card__body">
          <div class="post-card__meta">
            <time class="post-card__date" :datetime="post.created_at">
              {{ relativeTime(post.created_at) }}
            </time>
            <span v-if="post.status === 'draft'" class="post-card__badge">
              草稿
            </span>
          </div>

          <h2 class="post-card__title">{{ post.title }}</h2>

          <p v-if="post.excerpt" class="post-card__excerpt">
            {{ truncateExcerpt(post.excerpt) }}
          </p>

          <div class="post-card__footer">
            <span class="post-card__read-more">阅读全文 →</span>
          </div>
        </div>
      </article>
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
/* --- Home Page Layout --- */
.home {
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
.home__header {
  margin-bottom: var(--space-10);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.home__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.home__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Loading / Error / Empty States --- */
.home__loading,
.home__error,
.home__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.home__error p {
  color: var(--color-error);
  font-size: var(--text-base);
}

/* --- Post List --- */
.post-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

/* --- Post Card --- */
.post-card {
  display: flex;
  gap: var(--space-6);
  padding: var(--space-6) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  cursor: pointer;
  transition:
    background-color var(--transition-fast),
    padding-left var(--transition-fast);
  border-radius: var(--radius-md);
  position: relative;
}

.post-card:hover {
  background-color: var(--color-bg-sunken);
  padding-left: var(--space-6);
}

.post-card:hover .post-card__title {
  color: var(--color-accent);
}

.post-card:hover .post-card__read-more {
  opacity: 1;
  transform: translateX(0);
}

/* --- Cover Image --- */
.post-card__cover {
  flex-shrink: 0;
  width: 120px;
  height: 80px;
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--color-bg-sunken);
}

.post-card__cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-base);
}

.post-card:hover .post-card__cover img {
  transform: scale(1.05);
}

/* --- Card Body --- */
.post-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.post-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.post-card__date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.post-card__badge {
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-warning);
  background: rgba(196, 155, 62, 0.1);
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-card__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.4;
  transition: color var(--transition-fast);
  letter-spacing: -0.01em;
}

.post-card__excerpt {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-card__footer {
  margin-top: auto;
  padding-top: var(--space-1);
}

.post-card__read-more {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-accent);
  opacity: 0;
  transform: translateX(-8px);
  transition:
    opacity var(--transition-fast),
    transform var(--transition-fast);
  display: inline-block;
}

/* --- Pagination --- */
.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: var(--space-10);
  padding-top: var(--space-6);
  border-top: 1px solid var(--color-border-light);
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .home__title {
    font-size: var(--text-3xl);
  }

  .post-card {
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .post-card__cover {
    width: 100%;
    height: 160px;
    border-radius: var(--radius-md);
  }

  .post-card__read-more {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
