import { ref, reactive } from 'vue'
import api from '../lib/api'

/** Media file returned by the API */
export interface MediaFile {
  id: number
  filename: string
  original_name: string
  mime_type: string
  size_bytes: number
  url: string
  created_at: string
}

/** Paginated media list response */
export interface MediaListResponse {
  items: MediaFile[]
  total: number
  page: number
  per_page: number
}

/**
 * Media management composable.
 * Provides reactive state and methods for uploading, listing, and deleting media files.
 */
export function useMedia() {
  const files = ref<MediaFile[]>([])
  const loading = ref(false)
  const uploading = ref(false)
  const error = ref<string | null>(null)

  const pagination = reactive({
    page: 1,
    perPage: 20,
    total: 0,
  })

  /**
   * Fetch paginated media file list
   * @param page - Page number (1-based)
   * @param perPage - Items per page
   */
  async function fetchFiles(page = 1, perPage = 20): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const { data } = await api.get<MediaListResponse>('/api/v1/media', {
        params: { page, per_page: perPage },
      })

      files.value = data.items
      pagination.page = data.page
      pagination.perPage = data.per_page
      pagination.total = data.total
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch media files'
      error.value = message
      console.error('fetchFiles error:', err)
    } finally {
      loading.value = false
    }
  }

  /**
   * Upload a media file via multipart/form-data
   * @param file - The File object to upload
   * @returns The created MediaFile
   */
  async function uploadFile(file: File): Promise<MediaFile> {
    uploading.value = true
    error.value = null

    try {
      const formData = new FormData()
      formData.append('file', file)

      const { data } = await api.post<MediaFile>('/api/v1/media', formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
      })

      // Refresh the list to include the new file
      await fetchFiles(pagination.page, pagination.perPage)

      return data
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Upload failed'
      error.value = message
      console.error('uploadFile error:', err)
      throw err
    } finally {
      uploading.value = false
    }
  }

  /**
   * Delete a media file by ID
   * @param id - Media file ID
   */
  async function deleteFile(id: number): Promise<void> {
    loading.value = true
    error.value = null

    try {
      await api.delete(`/api/v1/media/${id}`)

      // Refresh the list after deletion
      await fetchFiles(pagination.page, pagination.perPage)
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to delete file'
      error.value = message
      console.error('deleteFile error:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * Format byte count to human-readable file size
   * @param bytes - File size in bytes
   */
  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B'
    const units = ['B', 'KB', 'MB', 'GB']
    const k = 1024
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${(bytes / Math.pow(k, i)).toFixed(1)} ${units[i]}`
  }

  /**
   * Format ISO date string to locale-readable date
   * @param isoDate - ISO 8601 date string
   */
  function formatDate(isoDate: string): string {
    const date = new Date(isoDate)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    })
  }

  /**
   * Check if a MIME type represents an image
   */
  function isImage(mimeType: string): boolean {
    return mimeType.startsWith('image/')
  }

  /**
   * Check if a MIME type represents a video
   */
  function isVideo(mimeType: string): boolean {
    return mimeType.startsWith('video/')
  }

  return {
    files,
    loading,
    uploading,
    error,
    pagination,
    fetchFiles,
    uploadFile,
    deleteFile,
    formatFileSize,
    formatDate,
    isImage,
    isVideo,
  }
}
