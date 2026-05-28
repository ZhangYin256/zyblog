import { ref } from 'vue'
import api from '../lib/api'

/** API 返回的标签结构 */
export interface Tag {
  id: number
  name: string
  slug: string
  created_at: string
}

/**
 * 标签相关 API 操作的组合式函数
 * 提供响应式状态和方法，用于获取、创建和管理标签及文章标签关联
 */
export function useTags() {
  const tags = ref<Tag[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 获取所有标签
   */
  async function fetchTags() {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<Tag[]>('/api/v1/tags')
      tags.value = data
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch tags'
      error.value = message
      console.error('fetchTags error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * 创建新标签
   * @param name - 标签名称
   */
  async function createTag(name: string) {
    try {
      const { data } = await api.post<Tag>('/api/v1/tags', { name })
      tags.value.push(data)
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to create tag'
      return { success: false, error: message }
    }
  }

  /**
   * 删除标签
   * @param id - 标签 ID
   */
  async function deleteTag(id: number) {
    try {
      await api.delete(`/api/v1/tags/${id}`)
      tags.value = tags.value.filter((t) => t.id !== id)
      return { success: true }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to delete tag'
      return { success: false, error: message }
    }
  }

  /**
   * 为文章分配标签
   * @param postId - 文章 ID
   * @param tagIds - 标签 ID 数组
   */
  async function assignTagsToPost(postId: number | string, tagIds: number[]) {
    try {
      const { data } = await api.post<Tag[]>(`/api/v1/posts/${postId}/tags`, {
        tag_ids: tagIds,
      })
      return { success: true, data }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to assign tags'
      return { success: false, error: message }
    }
  }

  /**
   * 获取文章的所有标签
   * @param postId - 文章 ID
   */
  async function fetchPostTags(postId: number | string): Promise<Tag[]> {
    try {
      const { data } = await api.get<Tag[]>(`/api/v1/posts/${postId}/tags`)
      return data
    } catch (err: unknown) {
      console.error('fetchPostTags error:', err)
      return []
    }
  }

  /**
   * 移除文章的某个标签
   * @param postId - 文章 ID
   * @param tagId - 标签 ID
   */
  async function removeTagFromPost(postId: number | string, tagId: number) {
    try {
      await api.delete(`/api/v1/posts/${postId}/tags/${tagId}`)
      return { success: true }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to remove tag'
      return { success: false, error: message }
    }
  }

  return {
    tags,
    loading,
    error,
    fetchTags,
    createTag,
    deleteTag,
    assignTagsToPost,
    fetchPostTags,
    removeTagFromPost,
  }
}
