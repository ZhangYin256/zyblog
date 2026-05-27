import { ref } from 'vue'
import api from '../lib/api'

/** PR 数据结构 */
export interface PullRequest {
  id: number
  post_id: number
  user_email: string
  content: string
  status: 'open' | 'closed' | 'merged'
  created_at: string
}

/** PR 列表响应 */
export interface PullListResponse {
  items: PullRequest[]
  total: number
}

/** 评论数据结构 */
export interface PullComment {
  id: number
  pull_request_id: number
  user_email: string
  content: string
  created_at: string
}

/** 创建 PR 请求 */
export interface CreatePullPayload {
  user_email: string
  content: string
}

/** 更新 PR 状态请求 */
export interface UpdatePullPayload {
  status: 'open' | 'closed' | 'merged'
}

/** 添加评论请求 */
export interface AddCommentPayload {
  user_email: string
  content: string
}

/**
 * Pull Request 相关 API 操作的组合式函数
 */
export function usePullRequests() {
  const pulls = ref<PullRequest[]>([])
  const currentPull = ref<PullRequest | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 获取文章的 PR 列表
   * @param postId - 文章 ID
   */
  async function fetchPulls(postId: number | string) {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<PullListResponse>(`/api/v1/posts/${postId}/pulls`)
      pulls.value = data.items
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch pull requests'
      error.value = message
      console.error('fetchPulls error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 创建新 PR
   * @param postId - 文章 ID
   * @param payload - PR 内容
   */
  async function createPull(postId: number | string, payload: CreatePullPayload) {
    try {
      const { data } = await api.post<PullRequest>(`/api/v1/posts/${postId}/pulls`, payload)
      // 添加到列表开头
      pulls.value.unshift(data)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to create pull request'
      return { success: false, error: message }
    }
  }

  /**
   * 更新 PR 状态
   * @param pullId - PR ID
   * @param payload - 新状态
   */
  async function updatePull(pullId: number, payload: UpdatePullPayload) {
    try {
      const { data } = await api.put<PullRequest>(`/api/v1/pulls/${pullId}`, payload)
      // 更新列表中的 PR
      const index = pulls.value.findIndex(p => p.id === pullId)
      if (index !== -1) {
        pulls.value[index] = data
      }
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to update pull request'
      return { success: false, error: message }
    }
  }

  /**
   * 添加评论
   * @param pullId - PR ID
   * @param payload - 评论内容
   */
  async function addComment(pullId: number, payload: AddCommentPayload) {
    try {
      const { data } = await api.post<PullComment>(`/api/v1/pulls/${pullId}/comments`, payload)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to add comment'
      return { success: false, error: message }
    }
  }

  /**
   * 格式化日期
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

  /**
   * 获取状态标签颜色
   * @param status - PR 状态
   */
  function getStatusColor(status: string): string {
    switch (status) {
      case 'open':
        return 'var(--color-success)'
      case 'closed':
        return 'var(--color-error)'
      case 'merged':
        return 'var(--color-info)'
      default:
        return 'var(--color-text-tertiary)'
    }
  }

  /**
   * 获取状态中文标签
   * @param status - PR 状态
   */
  function getStatusLabel(status: string): string {
    switch (status) {
      case 'open':
        return '开放'
      case 'closed':
        return '已关闭'
      case 'merged':
        return '已合并'
      default:
        return status
    }
  }

  return {
    // 状态
    pulls,
    currentPull,
    loading,
    error,

    // 方法
    fetchPulls,
    createPull,
    updatePull,
    addComment,
    formatDate,
    getStatusColor,
    getStatusLabel,
  }
}
