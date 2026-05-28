import axios from 'axios'
import type { AxiosRequestConfig, InternalAxiosRequestConfig } from 'axios'

const ACCESS_TOKEN_KEY = 'zyblog_admin_key'
const REFRESH_TOKEN_KEY = 'zyblog_refresh_token'

// Paths that should not have an Authorization header attached
const AUTH_ENDPOINTS = [
  '/api/v1/auth/login',
  '/api/v1/auth/register',
  '/api/v1/auth/refresh',
  '/api/v1/auth/github',
]

/**
 * Shared Axios instance for all API calls.
 *
 * - Attaches JWT access token (from localStorage) to every request
 *   except auth endpoints (login, register, refresh, github OAuth).
 * - On 401 responses, attempts a silent token refresh and retries once.
 */
const api = axios.create({
  baseURL: '',
  headers: {
    'Content-Type': 'application/json',
  },
})

// ── Request interceptor: attach JWT ──────────────────────

api.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const url = config.url ?? ''
  const isAuthEndpoint = AUTH_ENDPOINTS.some((path) => url.startsWith(path))

  if (!isAuthEndpoint) {
    const token = localStorage.getItem(ACCESS_TOKEN_KEY)
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
  }

  return config
})

// ── Response interceptor: 401 → refresh → retry ──────────

let isRefreshing = false
let refreshQueue: Array<{
  resolve: (token: string) => void
  reject: (err: unknown) => void
}> = []

function processQueue(error: unknown, token: string | null) {
  refreshQueue.forEach((pending) => {
    if (error || !token) {
      pending.reject(error)
    } else {
      pending.resolve(token)
    }
  })
  refreshQueue = []
}

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config as AxiosRequestConfig & { _retry?: boolean }

    // Only attempt refresh on 401, and only once per request
    if (error.response?.status !== 401 || originalRequest._retry) {
      return Promise.reject(error)
    }

    const url = originalRequest.url ?? ''
    const isAuthEndpoint = AUTH_ENDPOINTS.some((path) => url.startsWith(path))
    if (isAuthEndpoint) {
      return Promise.reject(error)
    }

    // If a refresh is already in progress, queue this request
    if (isRefreshing) {
      return new Promise((resolve, reject) => {
        refreshQueue.push({
          resolve: (newToken: string) => {
            if (originalRequest.headers) {
              originalRequest.headers.Authorization = `Bearer ${newToken}`
            }
            resolve(api(originalRequest))
          },
          reject,
        })
      })
    }

    originalRequest._retry = true
    isRefreshing = true

    try {
      const refreshToken = localStorage.getItem(REFRESH_TOKEN_KEY)
      if (!refreshToken) {
        throw new Error('No refresh token')
      }

      const { data } = await axios.post('/api/v1/auth/refresh', {
        refresh_token: refreshToken,
      })

      const newAccessToken: string = data.access_token
      const newRefreshToken: string = data.refresh_token

      localStorage.setItem(ACCESS_TOKEN_KEY, newAccessToken)
      localStorage.setItem(REFRESH_TOKEN_KEY, newRefreshToken)

      processQueue(null, newAccessToken)

      // Retry the original request with the new token
      if (originalRequest.headers) {
        originalRequest.headers.Authorization = `Bearer ${newAccessToken}`
      }
      return api(originalRequest)
    } catch (refreshError) {
      processQueue(refreshError, null)

      // Clear tokens on refresh failure
      localStorage.removeItem(ACCESS_TOKEN_KEY)
      localStorage.removeItem(REFRESH_TOKEN_KEY)

      // Refresh failed — send user to login
      window.location.href = '/login'
      return Promise.reject(refreshError)
    } finally {
      isRefreshing = false
    }
  },
)

export default api
