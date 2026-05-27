import axios from 'axios'

/**
 * 所有 API 调用共享的 axios 实例
 *
 * - 自动为写操作（POST、PUT、DELETE、PATCH）附加管理员 Bearer 令牌
 * - 令牌从 localStorage 的 'zyblog_admin_key' 键读取
 */
const api = axios.create({
  baseURL: '',
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器：为写方法附加认证令牌
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
