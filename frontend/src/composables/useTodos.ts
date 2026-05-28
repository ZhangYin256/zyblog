import { ref } from 'vue'
import api from '../lib/api'

/** TODO item from /api/v1/todos */
export interface TodoItem {
  id: number
  post_id: number
  title: string
  completed: boolean
  subscriber_count: number
}

/** API response for GET /api/v1/todos */
interface TodoListResponse {
  items: TodoItem[]
}

/**
 * TODO 管理相关 API 操作的组合式函数
 */
export function useTodos() {
  const todos = ref<TodoItem[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 获取所有 TODO 项
   */
  async function fetchTodos() {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<TodoListResponse>('/api/v1/todos')
      todos.value = data.items
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : '获取 TODO 列表失败'
      error.value = message
      console.error('fetchTodos error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 标记 TODO 为已完成
   * @param todoId - TODO 项 ID
   */
  async function completeTodo(todoId: number) {
    try {
      const { data } = await api.post<TodoItem>(`/api/v1/todos/${todoId}/complete`)
      // 更新列表中的对应项
      const idx = todos.value.findIndex((t) => t.id === todoId)
      if (idx !== -1) {
        todos.value[idx] = data
      }
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : '标记完成失败'
      return { success: false, error: message }
    }
  }

  return {
    todos,
    loading,
    error,
    fetchTodos,
    completeTodo,
  }
}
