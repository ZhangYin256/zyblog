import { ref, reactive } from 'vue'
import api from '../lib/api'

/** API 返回的文章结构 */
export interface Post {
  id: number
  title: string
  slug: string
  content: string
  excerpt: string | null
  cover_image: string | null
  status: 'published' | 'draft'
  created_at: string
  updated_at: string
}

/** 分页列表响应 */
export interface PostListResponse {
  items: Post[]
  total: number
  page: number
  per_page: number
}

/** API 返回的待办事项 */
export interface TodoItem {
  id: number
  post_id: number
  title: string
  description: string | null
  completed: boolean
  created_at: string
}

/** 订阅者请求负载 */
export interface CreateSubscriberPayload {
  email: string
  name?: string
}

/**
 * 文章相关 API 操作的组合式函数
 * 提供响应式状态和方法，用于获取、创建和管理文章
 */
export function usePosts() {
  const posts = ref<Post[]>([])
  const currentPost = ref<Post | null>(null)
  const postTodos = ref<TodoItem[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const pagination = reactive({
    page: 1,
    perPage: 10,
    total: 0,
  })

  /**
   * 获取分页文章列表
   * @param page - 页码（从 1 开始）
   * @param perPage - 每页条目数
   * @param status - 可选的状态筛选
   */
  async function fetchPosts(page = 1, perPage = 10, status?: string) {
    loading.value = true
    error.value = null

    try {
      const params: Record<string, string | number> = { page, per_page: perPage }
      if (status) params.status = status

      const { data } = await api.get<PostListResponse>('/api/v1/posts', { params })

      posts.value = data.items
      pagination.page = data.page
      pagination.perPage = data.per_page
      pagination.total = data.total
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch posts'
      error.value = message
      console.error('fetchPosts error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 根据 ID 获取单篇文章
   * @param id - 文章 ID
   */
  async function fetchPost(id: number | string) {
    loading.value = true
    error.value = null
    currentPost.value = null

    try {
      const { data } = await api.get<Post>(`/api/v1/posts/${id}`)
      currentPost.value = data
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch post'
      error.value = message
      console.error('fetchPost error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取特定文章的待办事项
   * @param id - 文章 ID
   */
  async function fetchPostTodos(id: number | string) {
    try {
      const { data } = await api.get<TodoItem[]>(`/api/v1/posts/${id}/todos`)
      postTodos.value = data
    } catch (err: unknown) {
      console.error('fetchPostTodos error:', err)
      postTodos.value = []
    }
  }

  /**
   * 创建新订阅者（邮箱订阅）
   * @param payload - 订阅者邮箱和可选名称
   */
  async function createSubscriber(payload: CreateSubscriberPayload) {
    try {
      const { data } = await api.post('/api/v1/subscribers', payload)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Subscription failed'
      return { success: false, error: message }
    }
  }

  /**
   * 将 ISO 日期字符串格式化为可读格式
   * @param isoDate - ISO 8601 日期字符串
   */
  function formatDate(isoDate: string): string {
    const date = new Date(isoDate)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  }

  /**
   * 获取草稿文章列表
   * @param page - 页码（从 1 开始）
   * @param perPage - 每页条目数
   */
  async function fetchDrafts(page = 1, perPage = 10) {
    return fetchPosts(page, perPage, 'draft')
  }

  /**
   * 发布草稿文章（将状态从 draft 改为 published）
   * @param id - 文章 ID
   */
  async function publishPost(id: number | string) {
    loading.value = true
    error.value = null

    try {
      await api.put(`/api/v1/posts/${id}`, { status: 'published' })
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to publish post'
      error.value = message
      console.error('publishPost error:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 删除文章
   * @param id - 文章 ID
   */
  async function deletePost(id: number | string) {
    loading.value = true
    error.value = null

    try {
      await api.delete(`/api/v1/posts/${id}`)
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to delete post'
      error.value = message
      console.error('deletePost error:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  return {
    // 状态
    posts,
    currentPost,
    postTodos,
    loading,
    error,
    pagination,

    // 方法
    fetchPosts,
    fetchPost,
    fetchDrafts,
    publishPost,
    deletePost,
    fetchPostTodos,
    createSubscriber,
    formatDate,
  }
}
