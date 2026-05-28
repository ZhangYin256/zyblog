<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NSpin, NEmpty, NButton, NPagination, NInput, NTag, NSpace, NSelect } from 'naive-ui'
import { useDebounceFn } from '@vueuse/core'
import { usePosts } from '../composables/usePosts'
import { useSearch } from '../composables/useSearch'
import { useTags } from '../composables/useTags'
import type { Post } from '../composables/usePosts'
import type { Tag } from '../composables/useTags'
import ArticleCard from '../components/ArticleCard.vue'

const router = useRouter()
const route = useRoute()
const {
  posts,
  loading,
  error,
  pagination,
  fetchPosts,
} = usePosts()

const {
  results: searchResults,
  loading: searchLoading,
  error: searchError,
  total: searchTotal,
  searched,
  searchPosts,
  clearSearch,
} = useSearch()

// 标签筛选
const {
  tags,
  fetchTags,
  fetchPostTags,
} = useTags()

// 文章标签缓存
const postTagsMap = ref<Map<number, Tag[]>>(new Map())

const searchQuery = ref('')
const isSearching = computed(() => searchQuery.value.trim().length > 0)
const activeTagId = ref<number | null>(null)
const filteredPosts = ref<Post[] | null>(null)

// 排序
type SortMode = 'newest' | 'oldest'
const sortBy = ref<SortMode>('newest')
const sortOptions = [
  { label: '最新发布', value: 'newest' },
  { label: '最早发布', value: 'oldest' },
]

// 根据 activeTagId 获取对应的 tag slug（用于 URL 同步）
const activeTagSlug = computed(() => {
  if (activeTagId.value === null) return null
  const tag = tags.value.find((t) => t.id === activeTagId.value)
  return tag?.slug ?? null
})

// 当前展示的文章列表：标签筛选 > 搜索模式 > 普通列表
const rawPosts = computed<Post[]>(() => {
  if (filteredPosts.value !== null) return filteredPosts.value
  return isSearching.value ? searchResults.value : posts.value
})

// 排序后的文章列表
const displayPosts = computed<Post[]>(() => {
  const list = [...rawPosts.value]
  list.sort((a, b) => {
    const dateA = new Date(a.created_at).getTime()
    const dateB = new Date(b.created_at).getTime()
    return sortBy.value === 'newest' ? dateB - dateA : dateA - dateB
  })
  return list
})

const displayLoading = computed(() =>
  isSearching.value ? searchLoading.value : loading.value
)
const displayError = computed(() =>
  isSearching.value ? searchError.value : error.value
)

// 计算总页数
const totalPages = computed(() =>
  Math.ceil(pagination.total / pagination.perPage)
)

// 防抖搜索函数（300ms）
const debouncedSearch = useDebounceFn((query: string) => {
  const queryObj: Record<string, string> = {}
  if (activeTagSlug.value) queryObj.tag = activeTagSlug.value
  if (sortBy.value !== 'newest') queryObj.sort = sortBy.value
  if (query.trim()) {
    searchPosts(query)
    queryObj.q = query
    router.replace({ query: queryObj })
  } else {
    clearSearch()
    router.replace({ query: queryObj })
  }
}, 300)

// 监听搜索输入变化
watch(searchQuery, (newQuery) => {
  debouncedSearch(newQuery)
})

// 同步排序到 URL
watch(sortBy, () => {
  syncToUrl()
})

// 当展示的文章列表变化时，加载标签
watch(displayPosts, (newPosts) => {
  if (newPosts.length > 0) {
    loadPostTags(newPosts)
  } else {
    postTagsMap.value = new Map()
  }
})

// 处理页码变化（仅普通列表模式）
function handlePageChange(page: number) {
  fetchPosts(page, pagination.perPage)
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// 清除搜索
function handleClearSearch() {
  searchQuery.value = ''
  clearSearch()
  syncToUrl()
}

// 切换标签筛选
async function toggleTag(tagId: number) {
  if (activeTagId.value === tagId) {
    // 取消选中
    activeTagId.value = null
    filteredPosts.value = null
    syncToUrl()
    return
  }

  activeTagId.value = tagId
  syncToUrl()

  // 获取当前展示文章的标签，筛选匹配的文章
  const postsToFilter = isSearching.value ? searchResults.value : posts.value
  const results = await Promise.all(
    postsToFilter.map(async (post) => {
      const postTags = await fetchPostTags(post.id)
      return { post, hasTag: postTags.some((t) => t.id === tagId) }
    })
  )
  filteredPosts.value = results.filter((r) => r.hasTag).map((r) => r.post)
}

// 重置标签筛选
function resetTagFilter() {
  activeTagId.value = null
  filteredPosts.value = null
  syncToUrl()
}

// 将状态同步到 URL query 参数
function syncToUrl() {
  const query: Record<string, string> = {}
  if (activeTagSlug.value) query.tag = activeTagSlug.value
  if (sortBy.value !== 'newest') query.sort = sortBy.value
  if (searchQuery.value.trim()) query.q = searchQuery.value.trim()
  router.replace({ query })
}

// 加载文章标签
async function loadPostTags(posts: Post[]) {
  const entries = await Promise.all(
    posts.map(async (post) => {
      const postTags = await fetchPostTags(post.id)
      return [post.id, postTags] as [number, Tag[]]
    })
  )
  postTagsMap.value = new Map(entries)
}

onMounted(() => {
  // 从 URL 恢复搜索状态
  const qParam = route.query.q
  if (typeof qParam === 'string' && qParam.trim()) {
    searchQuery.value = qParam
    searchPosts(qParam)
  }

  // 从 URL 恢复排序状态
  const sortParam = route.query.sort
  if (sortParam === 'oldest') {
    sortBy.value = 'oldest'
  }

  fetchPosts(1, 10, 'published')

  // 获取标签并从 URL 恢复选中状态
  fetchTags().then(() => {
    const tagParam = route.query.tag
    if (typeof tagParam === 'string' && tagParam.trim()) {
      const matchedTag = tags.value.find((t) => t.slug === tagParam)
      if (matchedTag) {
        toggleTag(matchedTag.id)
      }
    }
  })
})
</script>

<template>
  <div class="article-list">
    <!-- Header -->
    <header class="article-list__header">
      <h1 class="article-list__title">文章</h1>
      <p class="article-list__subtitle">浏览所有已发布的文章</p>
    </header>

    <!-- Controls: Search + Sort + Tags -->
    <div class="article-list__controls">
      <!-- Search Bar -->
      <div class="article-list__search">
        <n-input
          v-model:value="searchQuery"
          placeholder="搜索文章…"
          clearable
          size="large"
          :loading="searchLoading"
          @clear="handleClearSearch"
        >
          <template #prefix>
            <span class="search-icon">⌕</span>
          </template>
        </n-input>
        <p v-if="isSearching && searched && !searchLoading" class="article-list__search-meta">
          找到 {{ searchTotal }} 篇相关文章
        </p>
      </div>

      <!-- Sort Selector -->
      <div class="article-list__sort">
        <span class="article-list__sort-label">排序</span>
        <n-select
          v-model:value="sortBy"
          :options="sortOptions"
          size="medium"
          style="width: 140px"
        />
      </div>
    </div>

    <!-- Tag Filter -->
    <div v-if="tags.length > 0" class="article-list__tags">
      <span class="article-list__tags-label">按标签筛选</span>
      <n-space :size="8" align="center">
        <n-tag
          :checked="activeTagId === null"
          checkable
          round
          size="medium"
          @update:checked="resetTagFilter"
        >
          全部
        </n-tag>
        <n-tag
          v-for="tag in tags"
          :key="tag.id"
          checkable
          :checked="activeTagId === tag.id"
          round
          size="medium"
          @update:checked="toggleTag(tag.id)"
        >
          {{ tag.name }}
        </n-tag>
      </n-space>
    </div>

    <!-- Loading State -->
    <div v-if="displayLoading && displayPosts.length === 0" class="article-list__loading">
      <n-spin size="large" />
    </div>

    <!-- Error State -->
    <div v-else-if="displayError" class="article-list__error">
      <p>{{ displayError }}</p>
      <n-button v-if="!isSearching" @click="fetchPosts(1, 10, 'published')">重试</n-button>
      <n-button v-else @click="searchPosts(searchQuery)">重试</n-button>
    </div>

    <!-- Search Empty State -->
    <div v-else-if="isSearching && searched && searchResults.length === 0" class="article-list__empty">
      <n-empty description="未找到相关文章" />
      <n-button quaternary @click="handleClearSearch">清除搜索</n-button>
    </div>

    <!-- Normal Empty State -->
    <div v-else-if="!isSearching && posts.length === 0" class="article-list__empty">
      <n-empty description="暂无文章" />
    </div>

    <!-- Post List -->
    <div v-else class="article-grid">
      <ArticleCard
        v-for="post in displayPosts"
        :key="post.id"
        :post="post"
        :tags="postTagsMap.get(post.id)"
      />
    </div>

    <!-- Pagination (only in normal mode) -->
    <div v-if="!isSearching && totalPages > 1" class="pagination-wrapper">
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
/* --- Page Layout --- */
.article-list {
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
.article-list__header {
  margin-bottom: var(--space-8);
}

.article-list__title {
  font-family: var(--font-display);
  font-size: var(--text-3xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
  margin-bottom: var(--space-2);
}

.article-list__subtitle {
  font-size: var(--text-base);
  color: var(--color-text-tertiary);
}

/* --- Controls: Search + Sort --- */
.article-list__controls {
  display: flex;
  gap: var(--space-4);
  align-items: flex-start;
  margin-bottom: var(--space-6);
}

.article-list__search {
  flex: 1;
  min-width: 0;
}

.search-icon {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  line-height: 1;
}

.article-list__search-meta {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  margin-top: var(--space-2);
}

.article-list__sort {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.article-list__sort-label {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-weight: 400;
  white-space: nowrap;
}

/* --- Tag Filter --- */
.article-list__tags {
  margin-bottom: var(--space-6);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.article-list__tags-label {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  font-weight: 400;
}

/* --- Loading / Error / Empty --- */
.article-list__loading,
.article-list__error,
.article-list__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.article-list__error p {
  color: var(--color-error);
  font-size: var(--text-base);
}

/* --- Article Grid --- */
.article-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--space-6);
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
  .article-list__controls {
    flex-direction: column;
  }

  .article-list__sort {
    align-self: flex-end;
  }

  .article-grid {
    grid-template-columns: 1fr;
  }
}
</style>
