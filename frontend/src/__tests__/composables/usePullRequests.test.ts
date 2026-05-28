import { describe, it, expect, vi, beforeEach } from 'vitest'
import { ref } from 'vue'
import { usePullRequests } from '../../composables/usePullRequests'

vi.mock('../../lib/api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
  },
}))

import api from '../../lib/api'

describe('usePullRequests', () => {
  const postId = ref(1)

  beforeEach(() => {
    vi.clearAllMocks()
    postId.value = 1
  })

  it('initializes with empty state', () => {
    const { pulls, loading, error } = usePullRequests(postId)

    expect(pulls.value).toEqual([])
    expect(loading.value).toBe(false)
    expect(error.value).toBeNull()
  })

  it('fetchPulls sets loading state', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { loading, fetchPulls } = usePullRequests(postId)
    const promise = fetchPulls()

    expect(loading.value).toBe(true)

    await promise

    expect(loading.value).toBe(false)
  })

  it('fetchPulls populates pulls', async () => {
    const mockPulls = [
      {
        id: 1,
        post_id: 1,
        user_email: 'test@test.com',
        fragments: [{ start_line: 1, start_col: 0, end_line: 1, end_col: 5, replacement: 'Hello' }],
        message: 'Fix typo',
        status: 'open',
        created_at: '2024-01-01T00:00:00Z',
      },
    ]

    vi.mocked(api.get).mockResolvedValue({
      data: { items: mockPulls, total: 1 },
    })

    const { pulls, fetchPulls } = usePullRequests(postId)
    await fetchPulls()

    expect(pulls.value).toEqual(mockPulls)
  })

  it('fetchPulls calls correct API endpoint', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { fetchPulls } = usePullRequests(postId)
    await fetchPulls()

    expect(api.get).toHaveBeenCalledWith('/api/v1/posts/1/pulls')
  })

  it('fetchPulls uses reactive postId', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { fetchPulls } = usePullRequests(postId)
    postId.value = 42
    await fetchPulls()

    expect(api.get).toHaveBeenCalledWith('/api/v1/posts/42/pulls')
  })

  it('fetchPulls handles errors', async () => {
    vi.mocked(api.get).mockRejectedValue(new Error('Network error'))

    const { error, fetchPulls } = usePullRequests(postId)
    await fetchPulls()

    expect(error.value).toBe('Network error')
  })

  it('createPull calls correct API endpoint', async () => {
    vi.mocked(api.post).mockResolvedValue({ data: {} })
    vi.mocked(api.get).mockResolvedValue({ data: { items: [], total: 0 } })

    const { createPull } = usePullRequests(postId)
    const fragments = [{ start_line: 1, start_col: 0, end_line: 1, end_col: 5, replacement: 'Hello' }]
    await createPull(fragments, 'Fix typo', 'test@test.com')

    expect(api.post).toHaveBeenCalledWith('/api/v1/posts/1/pulls', {
      fragments,
      message: 'Fix typo',
      user_email: 'test@test.com',
    })
  })

  it('createPull refreshes pulls list after creation', async () => {
    vi.mocked(api.post).mockResolvedValue({ data: {} })
    vi.mocked(api.get).mockResolvedValue({ data: { items: [{ id: 1 }], total: 1 } })

    const { pulls, createPull } = usePullRequests(postId)
    const fragments = [{ start_line: 1, start_col: 0, end_line: 1, end_col: 5, replacement: 'Hello' }]
    await createPull(fragments, 'msg', 'test@test.com')

    expect(api.get).toHaveBeenCalled()
    expect(pulls.value).toEqual([{ id: 1 }])
  })

  it('applyPull calls correct API endpoint', async () => {
    vi.mocked(api.post).mockResolvedValue({ data: {} })
    vi.mocked(api.get).mockResolvedValue({ data: { items: [], total: 0 } })

    const { applyPull } = usePullRequests(postId)
    await applyPull(5)

    expect(api.post).toHaveBeenCalledWith('/api/v1/pulls/5/apply')
  })

  it('addComment calls correct API endpoint', async () => {
    vi.mocked(api.post).mockResolvedValue({ data: {} })

    const { addComment } = usePullRequests(postId)
    await addComment(3, 0, 5, 'nice', 'test@test.com')

    expect(api.post).toHaveBeenCalledWith('/api/v1/pulls/3/comments', {
      fragment_index: 0,
      line: 5,
      content: 'nice',
      user_email: 'test@test.com',
    })
  })

  it('fetchComments returns comments', async () => {
    const mockComments = [
      { id: 1, pull_request_id: 1, fragment_index: 0, line: 5, content: 'LGTM', user_email: 'a@b.com', created_at: '2024-01-01T00:00:00Z' },
    ]
    vi.mocked(api.get).mockResolvedValue({ data: mockComments })

    const { fetchComments } = usePullRequests(postId)
    const result = await fetchComments(1)

    expect(api.get).toHaveBeenCalledWith('/api/v1/pulls/1/comments')
    expect(result).toEqual(mockComments)
  })

  it('formatDate returns formatted date string', () => {
    const { formatDate } = usePullRequests(postId)
    const result = formatDate('2024-01-15T10:30:00Z')

    expect(result).toBeTruthy()
    expect(typeof result).toBe('string')
  })

  it('getStatusColor returns correct colors', () => {
    const { getStatusColor } = usePullRequests(postId)

    expect(getStatusColor('open')).toBeTruthy()
    expect(getStatusColor('closed')).toBeTruthy()
    expect(getStatusColor('merged')).toBeTruthy()
    expect(getStatusColor('unknown')).toBeTruthy()
  })

  it('getStatusLabel returns Chinese labels', () => {
    const { getStatusLabel } = usePullRequests(postId)

    expect(getStatusLabel('open')).toBe('开放')
    expect(getStatusLabel('closed')).toBe('已关闭')
    expect(getStatusLabel('merged')).toBe('已合并')
    expect(getStatusLabel('other')).toBe('other')
  })
})
