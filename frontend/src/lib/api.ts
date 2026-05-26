import axios from 'axios'

/**
 * Shared axios instance for all API calls.
 *
 * - Automatically attaches the admin Bearer token for write operations
 *   (POST, PUT, DELETE, PATCH).
 * - The token is read from localStorage key 'zyblog_admin_key'.
 */
const api = axios.create({
  baseURL: '',
  headers: {
    'Content-Type': 'application/json',
  },
})

// Request interceptor: attach auth token for write methods
api.interceptors.request.use((config) => {
  const method = config.method?.toUpperCase()
  if (method === 'POST' || method === 'PUT' || method === 'DELETE' || method === 'PATCH') {
    const token = localStorage.getItem('zyblog_admin_key')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
  }
  return config
})

export default api
