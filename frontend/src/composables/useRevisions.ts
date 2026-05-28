import { ref, type Ref } from 'vue'
import api from '../lib/api'

/** API 返回的修订版本结构 */
export interface Revision {
  id: number
  post_id: number
  title: string | null
  content: string | null
  excerpt: string | null
  cover_image: string | null
  version: number
  created_by: number | null
  created_at: string
}

/** 修订版本列表响应 */
interface RevisionListResponse {
  items: Revision[]
  total: number
}

/** Diff 响应 */
export interface DiffResponse {
  diff: string
  from_version: number
  to_version: number
}

/**
 * 修订版本管理组合式函数
 * 提供版本历史查看、Diff 对比和回滚功能
 */
export function useRevisions(postId: Ref<string>) {
  const revisions = ref<Revision[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 获取文章的修订版本列表
   */
  async function fetchRevisions(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<RevisionListResponse>(
        `/api/v1/posts/${postId.value}/revisions`,
      )
      revisions.value = data.items
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch revisions'
      error.value = message
      console.error('fetchRevisions error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取特定修订版本详情
   * @param revId - 修订版本 ID
   */
  async function getRevision(revId: number): Promise<Revision | null> {
    try {
      const { data } = await api.get<Revision>(
        `/api/v1/posts/${postId.value}/revisions/${revId}`,
      )
      return data
    } catch (err: unknown) {
      console.error('getRevision error:', err)
      return null
    }
  }

  /**
   * 回滚到指定修订版本
   * @param revId - 目标修订版本 ID
   */
  async function rollback(revId: number): Promise<{ success: boolean; error?: string }> {
    try {
      await api.post(`/api/v1/posts/${postId.value}/revisions/${revId}/rollback`)
      // 回滚成功后刷新修订列表
      await fetchRevisions()
      return { success: true }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Rollback failed'
      console.error('rollback error:', err)
      return { success: false, error: message }
    }
  }

  /**
   * 获取两个修订版本之间的差异
   * @param fromRevId - 源修订版本 ID
   * @param toRevId - 目标修订版本 ID
   */
  async function getDiff(
    fromRevId: number,
    toRevId: number,
  ): Promise<DiffResponse | null> {
    try {
      const { data } = await api.get<DiffResponse>(
        `/api/v1/posts/${postId.value}/diff`,
        { params: { from: fromRevId, to: toRevId } },
      )
      return data
    } catch (err: unknown) {
      console.error('getDiff error:', err)
      return null
    }
  }

  /**
   * 格式化日期为相对时间
   */
  function formatRelativeTime(isoDate: string): string {
    const date = new Date(isoDate)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffSec = Math.floor(diffMs / 1000)
    const diffMin = Math.floor(diffSec / 60)
    const diffHour = Math.floor(diffMin / 60)
    const diffDay = Math.floor(diffHour / 24)

    if (diffSec < 60) return '刚刚'
    if (diffMin < 60) return `${diffMin} 分钟前`
    if (diffHour < 24) return `${diffHour} 小时前`
    if (diffDay < 30) return `${diffDay} 天前`

    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    })
  }

  return {
    revisions,
    loading,
    error,
    fetchRevisions,
    getRevision,
    rollback,
    getDiff,
    formatRelativeTime,
  }
}
