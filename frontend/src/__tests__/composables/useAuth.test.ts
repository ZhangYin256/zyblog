import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockLocalStorage: Record<string, string> = {}

Object.defineProperty(window, 'localStorage', {
  value: {
    getItem: vi.fn((key: string) => mockLocalStorage[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      mockLocalStorage[key] = value
    }),
    removeItem: vi.fn((key: string) => {
      delete mockLocalStorage[key]
    }),
    clear: vi.fn(() => {
      Object.keys(mockLocalStorage).forEach(key => delete mockLocalStorage[key])
    }),
  },
})

import { useAuth } from '../../composables/useAuth'

describe('useAuth', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    Object.keys(mockLocalStorage).forEach(key => delete mockLocalStorage[key])
  })

  it('returns initial state', () => {
    const { isAuthenticated } = useAuth()

    expect(isAuthenticated.value).toBe(false)
  })

  it('setAdminKey stores key and updates auth state', () => {
    const { setAdminKey, isAuthenticated, getAdminKey } = useAuth()

    setAdminKey('test-key-123')

    expect(isAuthenticated.value).toBe(true)
    expect(getAdminKey()).toBe('test-key-123')
  })

  it('clearAdminKey removes key and updates auth state', () => {
    const { setAdminKey, clearAdminKey, isAuthenticated, getAdminKey } = useAuth()

    setAdminKey('test-key')
    clearAdminKey()

    expect(isAuthenticated.value).toBe(false)
    expect(getAdminKey()).toBeNull()
  })

  it('getAdminKey returns current key', () => {
    const { setAdminKey, getAdminKey } = useAuth()

    setAdminKey('my-secret')

    expect(getAdminKey()).toBe('my-secret')
  })

  it('persists key to localStorage', () => {
    const { setAdminKey } = useAuth()

    setAdminKey('persisted-key')

    expect(localStorage.setItem).toHaveBeenCalledWith('zyblog_admin_key', 'persisted-key')
  })

  it('clearAdminKey removes from localStorage', () => {
    const { setAdminKey, clearAdminKey } = useAuth()

    setAdminKey('temp-key')
    clearAdminKey()

    expect(localStorage.removeItem).toHaveBeenCalledWith('zyblog_admin_key')
  })
})
