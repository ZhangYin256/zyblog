import { ref } from 'vue'
import api from '../lib/api'
import type { Post } from './usePosts'

/** 搜索结果响应结构 */
export interface SearchResponse {
  items: Post[]
  total: number
  page: number
  per_page: number
}

/**
 * 文章搜索组合式函数
 * 提供响应式状态和方法，用于搜索文章
 */
export function useSearch() {
  const results = ref<Post[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const total = ref(0)
  const searched = ref(false)

  /**
   * 搜索文章
   * @param query - 搜索关键词
   * @param page - 页码（从 1 开始）
   * @param perPage - 每页条目数
   */
  async function searchPosts(query: string, page = 1, perPage = 10) {
    if (!query.trim()) {
      results.value = []
      total.value = 0
      searched.value = false
      return
    }

    loading.value = true
    error.value = null
    searched.value = true

    try {
      const { data } = await api.get<SearchResponse>('/api/v1/posts/search', {
        params: { q: query, page, per_page: perPage },
      })

      results.value = data.items
      total.value = data.total
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : '搜索失败'
      error.value = message
      results.value = []
      total.value = 0
      console.error('searchPosts error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 清除搜索状态
   */
  function clearSearch() {
    results.value = []
    total.value = 0
    error.value = null
    searched.value = false
  }

  return {
    results,
    loading,
    error,
    total,
    searched,
    searchPosts,
    clearSearch,
  }
}
