<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import type { Post } from '../composables/usePosts'
import type { Tag } from '../composables/useTags'

const props = defineProps<{
  post: Post
  tags?: Tag[]
}>()

const router = useRouter()

/** Navigate to post detail */
function goToPost() {
  router.push(`/posts/${props.post.id}`)
}

/** Relative time from ISO date string */
function relativeTime(isoDate: string): string {
  const now = new Date()
  const date = new Date(isoDate)
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) return '今天'
  if (diffDays === 1) return '昨天'
  if (diffDays < 7) return `${diffDays} 天前`
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} 周前`
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Truncate text to max length */
function truncate(text: string | null, maxLen = 120): string {
  if (!text) return ''
  if (text.length <= maxLen) return text
  return text.slice(0, maxLen).trimEnd() + '…'
}

const formattedDate = computed(() => relativeTime(props.post.created_at))
const displayExcerpt = computed(() => truncate(props.post.excerpt, 100))
const hasTags = computed(() => props.tags && props.tags.length > 0)
</script>

<template>
  <article class="article-card" @click="goToPost">
    <!-- Cover Image -->
    <div v-if="post.cover_image" class="article-card__cover">
      <img
        :src="post.cover_image"
        :alt="post.title"
        loading="lazy"
        class="article-card__image"
      />
    </div>

    <!-- Body -->
    <div class="article-card__body">
      <!-- Meta -->
      <div class="article-card__meta">
        <time class="article-card__date" :datetime="post.created_at">
          {{ formattedDate }}
        </time>
        <span v-if="post.status === 'draft'" class="article-card__badge">
          草稿
        </span>
      </div>

      <!-- Title -->
      <h3 class="article-card__title">{{ post.title }}</h3>

      <!-- Excerpt -->
      <p v-if="displayExcerpt" class="article-card__excerpt">
        {{ displayExcerpt }}
      </p>

      <!-- Tags -->
      <div v-if="hasTags" class="article-card__tags">
        <span
          v-for="tag in tags"
          :key="tag.id"
          class="article-card__tag"
        >
          {{ tag.name }}
        </span>
      </div>

      <!-- Footer -->
      <div class="article-card__footer">
        <span class="article-card__read-more">阅读全文 →</span>
      </div>
    </div>
  </article>
</template>

<style scoped>
.article-card {
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-lg);
  overflow: hidden;
  cursor: pointer;
  transition:
    transform var(--transition-fast),
    box-shadow var(--transition-fast),
    border-color var(--transition-fast);
}

.article-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
  border-color: var(--color-border);
}

/* --- Cover --- */
.article-card__cover {
  width: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  background: var(--color-bg-sunken);
}

.article-card__image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-base);
}

.article-card:hover .article-card__image {
  transform: scale(1.05);
}

/* --- Body --- */
.article-card__body {
  padding: var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  min-height: 160px;
}

/* --- Meta --- */
.article-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.article-card__date {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.article-card__badge {
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-warning);
  background: rgba(196, 155, 62, 0.1);
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* --- Title --- */
.article-card__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.4;
  letter-spacing: -0.01em;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  transition: color var(--transition-fast);
}

.article-card:hover .article-card__title {
  color: var(--color-accent);
}

/* --- Excerpt --- */
.article-card__excerpt {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* --- Tags --- */
.article-card__tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-1);
}

.article-card__tag {
  font-size: var(--text-xs);
  font-weight: 500;
  color: var(--color-accent);
  background: var(--color-accent-light);
  padding: 2px var(--space-2);
  border-radius: var(--radius-sm);
  line-height: 1.4;
}

/* --- Footer --- */
.article-card__footer {
  margin-top: auto;
  padding-top: var(--space-2);
}

.article-card__read-more {
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

.article-card:hover .article-card__read-more {
  opacity: 1;
  transform: translateX(0);
}

/* --- Mobile: always show read-more --- */
@media (max-width: 767px) {
  .article-card__read-more {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
