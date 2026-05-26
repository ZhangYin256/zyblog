import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import { ref } from 'vue'

const mockRoute = { params: { id: '1' } }
const mockRouter = { push: vi.fn() }

vi.mock('vue-router', () => ({
  useRoute: () => mockRoute,
  useRouter: () => mockRouter,
}))

vi.mock('naive-ui', () => ({
  useMessage: () => ({
    success: vi.fn(),
    error: vi.fn(),
    warning: vi.fn(),
  }),
  NSpin: { template: '<div class="spin" />' },
  NButton: { template: '<button><slot /></button>' },
  NInput: { template: '<input />' },
}))

const mockFetchPost = vi.fn()
const mockFetchPostTodos = vi.fn()
const mockCreateSubscriber = vi.fn()

vi.mock('../../composables/usePosts', () => ({
  usePosts: () => ({
    currentPost: ref(null),
    postTodos: ref([]),
    loading: ref(false),
    error: ref(null),
    fetchPost: mockFetchPost,
    fetchPostTodos: mockFetchPostTodos,
    createSubscriber: mockCreateSubscriber,
    formatDate: (d: string) => d,
  }),
}))

import PostDetail from '../../views/PostDetail.vue'

describe('PostDetail', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders the component', () => {
    const wrapper = mount(PostDetail, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.post-detail').exists()).toBe(true)
  })

  it('renders back button', () => {
    const wrapper = mount(PostDetail, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.back-button').exists()).toBe(true)
  })

  it('calls fetchPost on mount', () => {
    mount(PostDetail, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(mockFetchPost).toHaveBeenCalledWith('1')
  })

  it('calls fetchPostTodos on mount', async () => {
    mount(PostDetail, {
      global: { plugins: [createTestingPinia()] },
    })

    await new Promise(resolve => setTimeout(resolve, 0))
    expect(mockFetchPostTodos).toHaveBeenCalled()
  })

  it('navigates back on button click', async () => {
    const wrapper = mount(PostDetail, {
      global: { plugins: [createTestingPinia()] },
    })

    await wrapper.find('.back-button').trigger('click')

    expect(mockRouter.push).toHaveBeenCalledWith('/')
  })
})
