import { describe, it, expect, vi, beforeEach } from 'vitest'
import { usePosts } from '../../composables/usePosts'

vi.mock('../../lib/api', () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
  },
}))

import api from '../../lib/api'

describe('usePosts', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('initializes with empty state', () => {
    const { posts, currentPost, postTodos, loading, error } = usePosts()

    expect(posts.value).toEqual([])
    expect(currentPost.value).toBeNull()
    expect(postTodos.value).toEqual([])
    expect(loading.value).toBe(false)
    expect(error.value).toBeNull()
  })

  it('initializes pagination with defaults', () => {
    const { pagination } = usePosts()

    expect(pagination.page).toBe(1)
    expect(pagination.perPage).toBe(10)
    expect(pagination.total).toBe(0)
  })

  it('fetchPosts sets loading state', async () => {
    vi.mocked(api.get).mockResolvedValue({
      data: { items: [], total: 0, page: 1, per_page: 10 },
    })

    const { loading, fetchPosts } = usePosts()
    const promise = fetchPosts()

    expect(loading.value).toBe(true)

    await promise

    expect(loading.value).toBe(false)
  })

  it('fetchPosts populates posts', async () => {
    const mockPosts = [
      { id: 1, title: 'Post 1', slug: 'post-1', content: 'content', excerpt: null, cover_image: null, status: 'published', created_at: '2024-01-01', updated_at: '2024-01-01' },
    ]

    vi.mocked(api.get).mockResolvedValue({
      data: { items: mockPosts, total: 1, page: 1, per_page: 10 },
    })

    const { posts, fetchPosts } = usePosts()
    await fetchPosts()

    expect(posts.value).toEqual(mockPosts)
  })

  it('fetchPosts handles errors', async () => {
    vi.mocked(api.get).mockRejectedValue(new Error('Network error'))

    const { error, fetchPosts } = usePosts()
    await fetchPosts()

    expect(error.value).toBe('Network error')
  })

  it('fetchPost sets currentPost', async () => {
    const mockPost = { id: 1, title: 'Post 1', slug: 'post-1', content: 'content', excerpt: null, cover_image: null, status: 'published', created_at: '2024-01-01', updated_at: '2024-01-01' }

    vi.mocked(api.get).mockResolvedValue({ data: mockPost })

    const { currentPost, fetchPost } = usePosts()
    await fetchPost(1)

    expect(currentPost.value).toEqual(mockPost)
  })

  it('fetchPostTodos populates todos', async () => {
    const mockTodos = [
      { id: 1, post_id: 1, title: 'Todo 1', description: null, completed: false, created_at: '2024-01-01' },
    ]

    vi.mocked(api.get).mockResolvedValue({ data: mockTodos })

    const { postTodos, fetchPostTodos } = usePosts()
    await fetchPostTodos(1)

    expect(postTodos.value).toEqual(mockTodos)
  })

  it('createSubscriber returns success', async () => {
    vi.mocked(api.post).mockResolvedValue({ data: { id: 1 } })

    const { createSubscriber } = usePosts()
    const result = await createSubscriber({ email: 'test@example.com' })

    expect(result.success).toBe(true)
  })

  it('createSubscriber handles errors', async () => {
    vi.mocked(api.post).mockRejectedValue(new Error('Already subscribed'))

    const { createSubscriber } = usePosts()
    const result = await createSubscriber({ email: 'test@example.com' })

    expect(result.success).toBe(false)
    expect(result.error).toBe('Already subscribed')
  })

  it('formatDate returns formatted date string', () => {
    const { formatDate } = usePosts()
    const result = formatDate('2024-01-15T10:30:00Z')

    expect(result).toBeTruthy()
    expect(typeof result).toBe('string')
  })
})
