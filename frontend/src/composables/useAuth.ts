import { ref } from 'vue'
import api from '../lib/api'

// ─── Types ───────────────────────────────────────────────

export interface User {
  id: number
  email: string
  name: string
  role: string
  avatar_url: string | null
}

export interface AuthResponse {
  user: User
  access_token: string
  refresh_token: string
}

// ─── Constants ───────────────────────────────────────────

const ACCESS_TOKEN_KEY = 'zyblog_admin_key'
const REFRESH_TOKEN_KEY = 'zyblog_refresh_token'

// ─── Shared reactive state (singleton across component instances) ───

const user = ref<User | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const isAuthenticated = ref(!!localStorage.getItem(ACCESS_TOKEN_KEY))

// ─── Composable ──────────────────────────────────────────

/**
 * JWT-based authentication composable.
 *
 * Manages user state, login/register/logout flows, token refresh,
 * and GitHub OAuth. The access token is stored under the legacy
 * `zyblog_admin_key` key so existing admin-key consumers continue
 * to work until route guards land in T6.
 */
export function useAuth() {

  // ── Token helpers ──────────────────────────────────────

  function getAccessToken(): string | null {
    return localStorage.getItem(ACCESS_TOKEN_KEY)
  }

  function setTokens(accessToken: string, refreshToken: string) {
    localStorage.setItem(ACCESS_TOKEN_KEY, accessToken)
    localStorage.setItem(REFRESH_TOKEN_KEY, refreshToken)
    isAuthenticated.value = true
  }

  function clearTokens() {
    localStorage.removeItem(ACCESS_TOKEN_KEY)
    localStorage.removeItem(REFRESH_TOKEN_KEY)
    isAuthenticated.value = false
  }

  // ── Backward-compatible admin key API (used by Backup, Import, Publish) ──

  function setAdminKey(key: string) {
    localStorage.setItem(ACCESS_TOKEN_KEY, key)
    isAuthenticated.value = true
  }

  function clearAdminKey() {
    localStorage.removeItem(ACCESS_TOKEN_KEY)
    isAuthenticated.value = false
  }

  function getAdminKey(): string | null {
    return localStorage.getItem(ACCESS_TOKEN_KEY)
  }

  // ── Auth actions ───────────────────────────────────────

  /**
   * Register a new account.
   */
  async function register(email: string, password: string, name: string): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.post<AuthResponse>('/api/v1/auth/register', {
        email,
        password,
        name,
      })
      setTokens(data.access_token, data.refresh_token)
      user.value = data.user
    } catch (err: unknown) {
      const message = extractError(err, 'Registration failed')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * Log in with email and password.
   */
  async function login(email: string, password: string): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.post<AuthResponse>('/api/v1/auth/login', {
        email,
        password,
      })
      setTokens(data.access_token, data.refresh_token)
      user.value = data.user
    } catch (err: unknown) {
      const message = extractError(err, 'Login failed')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * Refresh the access token using the stored refresh token.
   */
  async function refreshToken(): Promise<void> {
    const refresh = localStorage.getItem(REFRESH_TOKEN_KEY)
    if (!refresh) {
      throw new Error('No refresh token available')
    }

    try {
      const { data } = await api.post<AuthResponse>('/api/v1/auth/refresh', {
        refresh_token: refresh,
      })
      setTokens(data.access_token, data.refresh_token)
      user.value = data.user
    } catch (err: unknown) {
      // Refresh failed — clear everything
      clearTokens()
      user.value = null
      throw err
    }
  }

  /**
   * Fetch the current user profile (GET /api/v1/auth/me).
   */
  async function fetchMe(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<User>('/api/v1/auth/me')
      user.value = data
    } catch (err: unknown) {
      const message = extractError(err, 'Failed to fetch user')
      error.value = message
      console.error('fetchMe error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Update the current user's profile (email, name).
   */
  async function updateProfile(email: string, name: string): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.put<User>('/api/v1/auth/profile', {
        email,
        name,
      })
      user.value = data
    } catch (err: unknown) {
      const message = extractError(err, 'Failed to update profile')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * Change the current user's password.
   */
  async function changePassword(currentPassword: string, newPassword: string): Promise<void> {
    loading.value = true
    error.value = null

    try {
      await api.put('/api/v1/auth/password', {
        current_password: currentPassword,
        new_password: newPassword,
      })
    } catch (err: unknown) {
      const message = extractError(err, 'Failed to change password')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * Log out — clear tokens and user state.
   */
  function logout(): void {
    clearTokens()
    user.value = null
  }

  /**
   * Build the GitHub OAuth redirect URL.
   */
  function getGitHubOAuthUrl(): string {
    return '/api/v1/auth/github'
  }

  // ── Helpers ────────────────────────────────────────────

  function extractError(err: unknown, fallback: string): string {
    if (err instanceof Error) return err.message
    if (
      typeof err === 'object' &&
      err !== null &&
      'response' in err &&
      typeof (err as Record<string, unknown>).response === 'object'
    ) {
      const resp = (err as { response?: { data?: { error?: string } } }).response
      if (resp?.data?.error) return resp.data.error
    }
    return fallback
  }

  return {
    // Reactive state
    user,
    loading,
    error,
    isAuthenticated,

    // Auth actions
    register,
    login,
    refreshToken,
    fetchMe,
    updateProfile,
    changePassword,
    logout,
    getGitHubOAuthUrl,

    // Token helpers (used by api.ts interceptor)
    getAccessToken,
    setTokens,

    // Backward-compatible admin key API
    setAdminKey,
    clearAdminKey,
    getAdminKey,
  }
}
