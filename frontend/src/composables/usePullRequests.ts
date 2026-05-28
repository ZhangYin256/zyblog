import { ref, type Ref } from 'vue'
import api from '../lib/api'

/** PR fragment — a single text replacement suggestion */
export interface Fragment {
  start_line: number
  start_col: number
  end_line: number
  end_col: number
  replacement: string
  description?: string
}

/** Pull Request with fragment-based content */
export interface PullRequest {
  id: number
  post_id: number
  fragments: Fragment[]
  message: string | null
  status: string
  user_email: string
  created_at: string
}

/** Comment on a PR fragment */
export interface Comment {
  id: number
  pull_request_id: number
  fragment_index: number
  line: number
  content: string
  user_email: string
  created_at: string
}

/** PR list API response */
export interface PullListResponse {
  items: PullRequest[]
  total: number
}

/**
 * Pull Request composable with fragment-based API.
 * Accepts postId as a reactive Ref for automatic tracking.
 */
export function usePullRequests(postId: Ref<number>) {
  const pulls = ref<PullRequest[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /** Fetch all PRs for the post */
  async function fetchPulls(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const { data } = await api.get<PullListResponse>(`/api/v1/posts/${postId.value}/pulls`)
      pulls.value = data.items
    } catch (err: unknown) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch pull requests'
      console.error('fetchPulls error:', err)
    } finally {
      loading.value = false
    }
  }

  /** Create a new PR with fragments */
  async function createPull(fragments: Fragment[], message: string, email: string): Promise<void> {
    await api.post(`/api/v1/posts/${postId.value}/pulls`, {
      fragments,
      message,
      user_email: email,
    })
    await fetchPulls()
  }

  /** Apply a PR (admin action — merges fragments into post) */
  async function applyPull(pullId: number): Promise<void> {
    await api.post(`/api/v1/pulls/${pullId}/apply`)
    await fetchPulls()
  }

  /** Add a comment on a specific fragment of a PR */
  async function addComment(
    pullId: number,
    fragmentIndex: number,
    line: number,
    content: string,
    email: string,
  ): Promise<void> {
    await api.post(`/api/v1/pulls/${pullId}/comments`, {
      fragment_index: fragmentIndex,
      line,
      content,
      user_email: email,
    })
  }

  /** Fetch comments for a PR */
  async function fetchComments(pullId: number): Promise<Comment[]> {
    const { data } = await api.get<Comment[]>(`/api/v1/pulls/${pullId}/comments`)
    return data
  }

  /** Format ISO date to Chinese locale string */
  function formatDate(isoDate: string): string {
    return new Date(isoDate).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  /** Map PR status to design token color */
  function getStatusColor(status: string): string {
    const map: Record<string, string> = {
      open: 'var(--color-success)',
      closed: 'var(--color-error)',
      merged: 'var(--color-info)',
    }
    return map[status] ?? 'var(--color-text-tertiary)'
  }

  /** Map PR status to Chinese label */
  function getStatusLabel(status: string): string {
    const map: Record<string, string> = {
      open: '开放',
      closed: '已关闭',
      merged: '已合并',
    }
    return map[status] ?? status
  }

  return {
    pulls,
    loading,
    error,
    fetchPulls,
    createPull,
    applyPull,
    addComment,
    fetchComments,
    formatDate,
    getStatusColor,
    getStatusLabel,
  }
}
