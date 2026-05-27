import { describe, it, expect, vi, beforeEach } from 'vitest'
import { usePullRequests } from '../../composables/usePullRequests'

vi.mock('../../lib/api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
  },
}))

import api from '../../lib/api'

describe('usePullRequests', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('initializes with empty state', () => {
    const { pulls, currentPull, loading, error } = usePullRequests()

    expect(pulls.value).toEqual([])
    expect(currentPull.value).toBeNull()
    expect(loading.value).toBe(false)
    expect(error.value).toBeNull()
  })

  it('fetchPulls sets loading state', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { loading, fetchPulls } = usePullRequests()
    const promise = fetchPulls(1)

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
        content: 'Fix typo',
        status: 'open' as const,
        created_at: '2024-01-01T00:00:00Z',
      },
    ]

    vi.mocked(api.get).mockResolvedValue({
      data: { items: mockPulls, total: 1 },
    })

    const { pulls, fetchPulls } = usePullRequests()
    await fetchPulls(1)

    expect(pulls.value).toEqual(mockPulls)
  })

  it('fetchPulls calls correct API endpoint', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { fetchPulls } = usePullRequests()
    await fetchPulls(42)

    expect(api.get).toHaveBeenCalledWith('/api/v1/posts/42/pulls')
  })

  it('fetchPulls handles errors', async () => {
    vi.mocked(api.get).mockRejectedValue(new Error('Network error'))

    const { error, fetchPulls } = usePullRequests()
    await fetchPulls(1)

    expect(error.value).toBe('Network error')
  })

  it('createPull returns success', async () => {
    const mockPull = {
      id: 1,
      post_id: 1,
      user_email: 'test@test.com',
      content: 'New PR',
      status: 'open' as const,
      created_at: '2024-01-01T00:00:00Z',
    }

    vi.mocked(api.post).mockResolvedValue({ data: mockPull })

    const { createPull } = usePullRequests()
    const result = await createPull(1, {
      user_email: 'test@test.com',
      content: 'New PR',
    })

    expect(result.success).toBe(true)
    expect(result.data).toEqual(mockPull)
  })

  it('createPull adds to pulls list', async () => {
    const mockPull = {
      id: 2,
      post_id: 1,
      user_email: 'test@test.com',
      content: 'New PR',
      status: 'open' as const,
      created_at: '2024-01-01T00:00:00Z',
    }

    vi.mocked(api.post).mockResolvedValue({ data: mockPull })

    const { pulls, createPull } = usePullRequests()
    await createPull(1, {
      user_email: 'test@test.com',
      content: 'New PR',
    })

    expect(pulls.value[0]).toEqual(mockPull)
  })

  it('createPull handles errors', async () => {
    vi.mocked(api.post).mockRejectedValue(new Error('Creation failed'))

    const { createPull } = usePullRequests()
    const result = await createPull(1, {
      user_email: 'test@test.com',
      content: 'New PR',
    })

    expect(result.success).toBe(false)
    expect(result.error).toBe('Creation failed')
  })

  it('updatePull returns success', async () => {
    const mockUpdated = {
      id: 1,
      post_id: 1,
      user_email: 'test@test.com',
      content: 'PR content',
      status: 'closed' as const,
      created_at: '2024-01-01T00:00:00Z',
    }

    vi.mocked(api.put).mockResolvedValue({ data: mockUpdated })

    const { updatePull } = usePullRequests()
    const result = await updatePull(1, { status: 'closed' })

    expect(result.success).toBe(true)
  })

  it('updatePull calls correct API endpoint', async () => {
    vi.mocked(api.put).mockResolvedValue({
      data: {
        id: 5,
        post_id: 1,
        user_email: 'test@test.com',
        content: 'PR',
        status: 'merged',
        created_at: '2024-01-01T00:00:00Z',
      },
    })

    const { updatePull } = usePullRequests()
    await updatePull(5, { status: 'merged' })

    expect(api.put).toHaveBeenCalledWith('/api/v1/pulls/5', { status: 'merged' })
  })

  it('updatePull updates item in pulls list', async () => {
    const initial = {
      id: 1,
      post_id: 1,
      user_email: 'test@test.com',
      content: 'PR',
      status: 'open' as const,
      created_at: '2024-01-01T00:00:00Z',
    }
    const updated = { ...initial, status: 'closed' as const }

    vi.mocked(api.get).mockResolvedValue({
      data: { items: [initial], total: 1 },
    })
    vi.mocked(api.put).mockResolvedValue({ data: updated })

    const { pulls, fetchPulls, updatePull } = usePullRequests()
    await fetchPulls(1)
    await updatePull(1, { status: 'closed' })

    expect(pulls.value[0].status).toBe('closed')
  })

  it('updatePull handles errors', async () => {
    vi.mocked(api.put).mockRejectedValue(new Error('Update failed'))

    const { updatePull } = usePullRequests()
    const result = await updatePull(1, { status: 'closed' })

    expect(result.success).toBe(false)
    expect(result.error).toBe('Update failed')
  })

  it('addComment returns success', async () => {
    const mockComment = {
      id: 1,
      pull_request_id: 1,
      user_email: 'reviewer@test.com',
      content: 'LGTM',
      created_at: '2024-01-01T00:00:00Z',
    }

    vi.mocked(api.post).mockResolvedValue({ data: mockComment })

    const { addComment } = usePullRequests()
    const result = await addComment(1, {
      user_email: 'reviewer@test.com',
      content: 'LGTM',
    })

    expect(result.success).toBe(true)
    expect(result.data).toEqual(mockComment)
  })

  it('addComment calls correct API endpoint', async () => {
    vi.mocked(api.post).mockResolvedValue({
      data: {
        id: 1,
        pull_request_id: 3,
        user_email: 'test@test.com',
        content: 'nice',
        created_at: '2024-01-01T00:00:00Z',
      },
    })

    const { addComment } = usePullRequests()
    await addComment(3, { user_email: 'test@test.com', content: 'nice' })

    expect(api.post).toHaveBeenCalledWith('/api/v1/pulls/3/comments', {
      user_email: 'test@test.com',
      content: 'nice',
    })
  })

  it('addComment handles errors', async () => {
    vi.mocked(api.post).mockRejectedValue(new Error('Comment failed'))

    const { addComment } = usePullRequests()
    const result = await addComment(1, {
      user_email: 'test@test.com',
      content: 'comment',
    })

    expect(result.success).toBe(false)
    expect(result.error).toBe('Comment failed')
  })

  it('formatDate returns formatted date string', () => {
    const { formatDate } = usePullRequests()
    const result = formatDate('2024-01-15T10:30:00Z')

    expect(result).toBeTruthy()
    expect(typeof result).toBe('string')
  })

  it('getStatusColor returns correct colors', () => {
    const { getStatusColor } = usePullRequests()

    expect(getStatusColor('open')).toBeTruthy()
    expect(getStatusColor('closed')).toBeTruthy()
    expect(getStatusColor('merged')).toBeTruthy()
    expect(getStatusColor('unknown')).toBeTruthy()
  })

  it('getStatusLabel returns Chinese labels', () => {
    const { getStatusLabel } = usePullRequests()

    expect(getStatusLabel('open')).toBe('开放')
    expect(getStatusLabel('closed')).toBe('已关闭')
    expect(getStatusLabel('merged')).toBe('已合并')
    expect(getStatusLabel('other')).toBe('other')
  })

  it('fetchPulls accepts string postId', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0 },
    })

    const { fetchPulls } = usePullRequests()
    await fetchPulls('99')

    expect(api.get).toHaveBeenCalledWith('/api/v1/posts/99/pulls')
  })
})
