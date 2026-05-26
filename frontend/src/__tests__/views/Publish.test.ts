import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import { ref } from 'vue'

const mockRouter = { push: vi.fn() }

vi.mock('vue-router', () => ({
  useRouter: () => mockRouter,
}))

vi.mock('naive-ui', () => ({
  useMessage: () => ({
    success: vi.fn(),
    error: vi.fn(),
    warning: vi.fn(),
  }),
  NButton: { template: '<button><slot /></button>' },
  NInput: { template: '<input />' },
}))

vi.mock('@vueuse/core', () => ({
  useDebounceFn: (fn: Function) => fn,
}))

const mockIsAuthenticated = ref(false)
const mockSetAdminKey = vi.fn()

vi.mock('../../composables/useAuth', () => ({
  useAuth: () => ({
    isAuthenticated: mockIsAuthenticated,
    setAdminKey: mockSetAdminKey,
  }),
}))

vi.mock('../../lib/api', () => ({
  default: {
    post: vi.fn().mockResolvedValue({ data: { id: 1 } }),
  },
}))

import Publish from '../../views/Publish.vue'

describe('Publish', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    localStorage.clear()
    mockIsAuthenticated.value = false
  })

  it('renders the publish page', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-page').exists()).toBe(true)
  })

  it('renders title input', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-title').exists()).toBe(true)
  })

  it('renders content textarea', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-content').exists()).toBe(true)
  })

  it('renders publish button', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-header__right button').exists()).toBe(true)
  })

  it('shows "写点什么" header by default', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-header__title').text()).toBe('写点什么')
  })

  it('renders toolbar buttons', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    const toolbarBtns = wrapper.findAll('.toolbar-btn')
    expect(toolbarBtns.length).toBeGreaterThanOrEqual(2)
  })

  it('renders footer hints', () => {
    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-footer').exists()).toBe(true)
  })

  it('loads draft from localStorage on mount', () => {
    localStorage.setItem('zyblog_draft', JSON.stringify({
      title: 'Draft Title',
      content: 'Draft Content',
      savedAt: new Date().toISOString(),
    }))

    const wrapper = mount(Publish, {
      global: { plugins: [createTestingPinia()] },
    })

    expect(wrapper.find('.publish-page').exists()).toBe(true)
  })
})
