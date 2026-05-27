import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import VideoPlayer from '../../components/VideoPlayer.vue'

describe('VideoPlayer', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders with src prop', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('.video-player').exists()).toBe(true)
  })

  it('renders video element', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    const video = wrapper.find('video')
    expect(video.exists()).toBe(true)
    expect(video.attributes('src')).toBe('/static/videos/test.mp4')
  })

  it('sets preload to metadata', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('video').attributes('preload')).toBe('metadata')
  })

  it('sets playsinline attribute', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('video').attributes('playsinline')).toBeDefined()
  })

  it('renders play overlay by default', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('.video-player__overlay').exists()).toBe(true)
    expect(wrapper.find('.video-player__play-btn').text()).toBe('▶')
  })

  it('does not show error state by default', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('.video-player__error').exists()).toBe(false)
  })

  it('uses provided type prop', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4', type: 'video/webm' },
    })

    expect(wrapper.find('video').attributes('type')).toBe('video/webm')
  })

  it('infers mp4 type from extension', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    expect(wrapper.find('video').attributes('type')).toBe('video/mp4')
  })

  it('infers webm type from extension', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.webm' },
    })

    expect(wrapper.find('video').attributes('type')).toBe('video/webm')
  })

  it('infers ogg type from extension', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.ogg' },
    })

    expect(wrapper.find('video').attributes('type')).toBe('video/ogg')
  })

  it('defaults to mp4 for unknown extension', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.xyz' },
    })

    expect(wrapper.find('video').attributes('type')).toBe('video/mp4')
  })

  it('applies autoplay prop', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4', autoplay: true },
    })

    expect(wrapper.find('video').attributes('autoplay')).toBeDefined()
  })

  it('applies loop prop', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4', loop: true },
    })

    expect(wrapper.find('video').attributes('loop')).toBeDefined()
  })

  it('applies muted prop', () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4', muted: true },
    })

    expect(wrapper.find('video').attributes('muted')).toBeDefined()
  })

  it('shows error state when video errors', async () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    await wrapper.find('video').trigger('error')

    expect(wrapper.find('.video-player__error').exists()).toBe(true)
    expect(wrapper.find('.video-player__error-text').text()).toBe('视频加载失败')
  })

  it('hides video element when error occurs', async () => {
    const wrapper = mount(VideoPlayer, {
      props: { src: '/static/videos/test.mp4' },
    })

    await wrapper.find('video').trigger('error')

    expect(wrapper.find('video').exists()).toBe(false)
  })
})
