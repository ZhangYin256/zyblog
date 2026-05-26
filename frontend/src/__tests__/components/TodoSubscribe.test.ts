import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import TodoSubscribe from '../../components/TodoSubscribe.vue'

vi.mock('naive-ui', () => ({
  useMessage: () => ({
    success: vi.fn(),
    error: vi.fn(),
    warning: vi.fn(),
  }),
  NInput: { template: '<input />' },
  NButton: { template: '<button><slot /></button>' },
  NTag: { template: '<span><slot /></span>' },
  NDivider: { template: '<hr />' },
}))

vi.mock('../../lib/api', () => ({
  default: {
    post: vi.fn(),
  },
}))

describe('TodoSubscribe', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders with default props', () => {
    const wrapper = mount(TodoSubscribe, {
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe').exists()).toBe(true)
    expect(wrapper.find('.todo-subscribe__title').text()).toContain('待办事项订阅')
  })

  it('renders tags when provided', () => {
    const wrapper = mount(TodoSubscribe, {
      props: {
        tags: ['bug-fix', 'feature'],
      },
      global: {
        plugins: [createTestingPinia()],
      },
    })

    const tags = wrapper.findAll('.todo-subscribe__tag')
    expect(tags.length).toBe(2)
  })

  it('adds # prefix to tags without it', () => {
    const wrapper = mount(TodoSubscribe, {
      props: {
        tags: ['bug-fix'],
      },
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__tag').text()).toContain('#bug-fix')
  })

  it('does not double-prefix tags already starting with #', () => {
    const wrapper = mount(TodoSubscribe, {
      props: {
        tags: ['#already-prefixed'],
      },
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__tag').text()).toBe('#already-prefixed')
  })

  it('shows subscriber count when provided', () => {
    const wrapper = mount(TodoSubscribe, {
      props: {
        subscriberCount: 42,
      },
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__count-number').text()).toBe('42')
  })

  it('formats large subscriber count with k suffix', () => {
    const wrapper = mount(TodoSubscribe, {
      props: {
        subscriberCount: 1500,
      },
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__count-number').text()).toBe('1.5k')
  })

  it('hides subscriber count when not provided', () => {
    const wrapper = mount(TodoSubscribe, {
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__count').exists()).toBe(false)
  })

  it('shows form when not subscribed', () => {
    const wrapper = mount(TodoSubscribe, {
      global: {
        plugins: [createTestingPinia()],
      },
    })

    expect(wrapper.find('.todo-subscribe__form').exists()).toBe(true)
    expect(wrapper.find('.todo-subscribe__success').exists()).toBe(false)
  })

  it('validates empty email', async () => {
    const wrapper = mount(TodoSubscribe, {
      global: {
        plugins: [createTestingPinia()],
      },
    })

    await wrapper.find('.todo-subscribe__button').trigger('click')

    expect(wrapper.find('.todo-subscribe__error').exists()).toBe(true)
  })
})
