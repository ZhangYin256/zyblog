import { ref } from 'vue'
import api from '../lib/api'

/** 评论数据结构 */
export interface Comment {
  id: number
  post_id: number
  author_name: string
  author_email: string | null
  content: string
  approved: boolean
  referenced_content: string | null
  created_at: string
  updated_at: string
}

/** 提交评论请求 */
export interface CreateCommentPayload {
  author_name: string
  author_email?: string
  content: string
  referenced_content?: string
}

/**
 * 评论相关 API 操作的组合式函数
 */
export function useComments() {
  const comments = ref<Comment[]>([])
  const pendingComments = ref<Comment[]>([])
  const loading = ref(false)
  const submitting = ref(false)
  const error = ref<string | null>(null)

  /**
   * 获取文章已审核评论（公开）
   * @param postId - 文章 ID
   */
  async function fetchApprovedComments(postId: number | string) {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<Comment[]>(`/api/v1/posts/${postId}/comments`)
      comments.value = data
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch comments'
      error.value = message
      console.error('fetchApprovedComments error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 提交评论（公开，无需认证）
   * @param postId - 文章 ID
   * @param payload - 评论内容
   */
  async function submitComment(postId: number | string, payload: CreateCommentPayload) {
    submitting.value = true

    try {
      const { data } = await api.post<Comment>(`/api/v1/posts/${postId}/comments`, payload)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to submit comment'
      return { success: false, error: message }
    } finally {
      submitting.value = false
    }
  }

  /**
   * 获取待审核评论列表（需认证）
   */
  async function fetchPendingComments() {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<Comment[]>('/api/v1/comments/pending')
      pendingComments.value = data
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch pending comments'
      error.value = message
      console.error('fetchPendingComments error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 审核通过评论（需认证）
   * @param commentId - 评论 ID
   */
  async function approveComment(commentId: number) {
    try {
      const { data } = await api.put<Comment>(`/api/v1/comments/${commentId}/approve`)
      // 从待审列表移除
      pendingComments.value = pendingComments.value.filter(c => c.id !== commentId)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to approve comment'
      return { success: false, error: message }
    }
  }

  /**
   * 删除评论（需认证）
   * @param commentId - 评论 ID
   */
  async function deleteComment(commentId: number) {
    try {
      await api.delete(`/api/v1/comments/${commentId}`)
      // 从待审列表移除
      pendingComments.value = pendingComments.value.filter(c => c.id !== commentId)
      // 从已审列表移除
      comments.value = comments.value.filter(c => c.id !== commentId)
      return { success: true }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to delete comment'
      return { success: false, error: message }
    }
  }

  /**
   * 格式化为相对时间
   * @param isoDate - ISO 8601 日期字符串
   */
  function formatRelativeTime(isoDate: string): string {
    const now = new Date()
    const date = new Date(isoDate)
    const diffMs = now.getTime() - date.getTime()
    const diffSeconds = Math.floor(diffMs / 1000)
    const diffMinutes = Math.floor(diffSeconds / 60)
    const diffHours = Math.floor(diffMinutes / 60)
    const diffDays = Math.floor(diffHours / 24)

    if (diffSeconds < 60) return '刚刚'
    if (diffMinutes < 60) return `${diffMinutes} 分钟前`
    if (diffHours < 24) return `${diffHours} 小时前`
    if (diffDays < 7) return `${diffDays} 天前`
    if (diffDays < 30) return `${Math.floor(diffDays / 7)} 周前`

    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  }

  /**
   * 格式化完整日期
   * @param isoDate - ISO 8601 日期字符串
   */
  function formatDate(isoDate: string): string {
    const date = new Date(isoDate)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  return {
    // 状态
    comments,
    pendingComments,
    loading,
    submitting,
    error,

    // 方法
    fetchApprovedComments,
    submitComment,
    fetchPendingComments,
    approveComment,
    deleteComment,
    formatRelativeTime,
    formatDate,
  }
}
