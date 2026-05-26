import { ref, reactive } from 'vue'
import api from '../lib/api'

/** Post shape returned by the API */
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

/** Paginated list response */
export interface PostListResponse {
  items: Post[]
  total: number
  page: number
  per_page: number
}

/** Todo item returned by the API */
export interface TodoItem {
  id: number
  post_id: number
  title: string
  description: string | null
  completed: boolean
  created_at: string
}

/** Subscriber request payload */
export interface CreateSubscriberPayload {
  email: string
  name?: string
}

/**
 * Composable for post-related API operations.
 * Provides reactive state and methods for fetching, creating, and managing posts.
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
   * Fetch paginated post list.
   * @param page - Page number (1-based)
   * @param perPage - Items per page
   * @param status - Optional filter by status
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
   * Fetch a single post by ID.
   * @param id - Post ID
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
   * Fetch todo items for a specific post.
   * @param id - Post ID
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
   * Create a new subscriber (email subscription).
   * @param payload - Subscriber email and optional name
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
   * Format ISO date string to a readable format.
   * @param isoDate - ISO 8601 date string
   */
  function formatDate(isoDate: string): string {
    const date = new Date(isoDate)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  }

  return {
    // State
    posts,
    currentPost,
    postTodos,
    loading,
    error,
    pagination,

    // Methods
    fetchPosts,
    fetchPost,
    fetchPostTodos,
    createSubscriber,
    formatDate,
  }
}
