<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import {
  NButton,
  NSpin,
  NEmpty,
  NAlert,
  NPopconfirm,
  NPagination,
  NTag,
  useMessage,
} from 'naive-ui'
import { useMedia, type MediaFile } from '../composables/useMedia'

const message = useMessage()
const {
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
} = useMedia()

// Drag-and-drop state
const isDragOver = ref(false)
const fileInputRef = ref<HTMLInputElement | null>(null)

// Preview modal
const previewFile = ref<MediaFile | null>(null)

const hasFiles = computed(() => files.value.length > 0)
const totalPages = computed(() => Math.ceil(pagination.total / pagination.perPage))

// ── Upload handlers ──────────────────────────────────────

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = true
}

function handleDragLeave() {
  isDragOver.value = false
}

async function handleDrop(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = false

  const droppedFiles = e.dataTransfer?.files
  if (droppedFiles && droppedFiles.length > 0) {
    await uploadFiles(Array.from(droppedFiles))
  }
}

function triggerFileInput() {
  fileInputRef.value?.click()
}

async function handleFileSelect(e: Event) {
  const target = e.target as HTMLInputElement
  const selectedFiles = target.files
  if (selectedFiles && selectedFiles.length > 0) {
    await uploadFiles(Array.from(selectedFiles))
    // Reset input so the same file can be re-selected
    target.value = ''
  }
}

async function uploadFiles(fileList: File[]) {
  for (const file of fileList) {
    try {
      await uploadFile(file)
      message.success(`${file.name} 上传成功`)
    } catch {
      message.error(`${file.name} 上传失败`)
    }
  }
}

// ── Delete handler ───────────────────────────────────────

async function handleDelete(file: MediaFile) {
  try {
    await deleteFile(file.id)
    message.success('文件已删除')
  } catch {
    message.error('删除失败')
  }
}

// ── Preview ──────────────────────────────────────────────

function openPreview(file: MediaFile) {
  previewFile.value = file
}

function closePreview() {
  previewFile.value = null
}

// ── Pagination ───────────────────────────────────────────

function handlePageChange(page: number) {
  fetchFiles(page, pagination.perPage)
}

// ── Lifecycle ────────────────────────────────────────────

onMounted(() => {
  fetchFiles()
})
</script>

<template>
  <div class="media-page">
    <!-- Header -->
    <header class="media-header">
      <div class="media-header__info">
        <h2 class="media-title">媒体管理</h2>
        <p class="media-subtitle">上传和管理图片、视频等媒体文件</p>
      </div>
      <n-button
        type="primary"
        :loading="uploading"
        @click="triggerFileInput"
      >
        {{ uploading ? '上传中...' : '上传文件' }}
      </n-button>
    </header>

    <!-- Hidden file input -->
    <input
      ref="fileInputRef"
      type="file"
      accept="image/jpeg,image/png,image/gif,image/webp,video/mp4,video/webm"
      multiple
      class="media-hidden-input"
      @change="handleFileSelect"
    />

    <!-- Error Alert -->
    <n-alert
      v-if="error"
      type="error"
      :bordered="false"
      closable
      @close="error = null"
      class="media-alert"
    >
      {{ error }}
    </n-alert>

    <!-- Upload Drop Zone -->
    <div
      class="media-dropzone"
      :class="{ 'media-dropzone--active': isDragOver }"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
      @click="triggerFileInput"
    >
      <div class="media-dropzone__content">
        <span class="media-dropzone__icon">{{ isDragOver ? '↓' : '↑' }}</span>
        <p class="media-dropzone__text">
          {{ isDragOver ? '松开上传文件' : '拖放文件到此处或点击上传' }}
        </p>
        <p class="media-dropzone__hint">
          支持 JPEG、PNG、GIF、WebP、MP4、WebM（最大 100MB）
        </p>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading && !hasFiles" class="media-loading">
      <n-spin size="large" />
      <span>加载媒体文件...</span>
    </div>

    <!-- Empty State -->
    <n-empty
      v-else-if="!hasFiles && !loading"
      description="暂无媒体文件"
      class="media-empty"
    >
      <template #extra>
        <n-button type="primary" @click="triggerFileInput">
          上传第一个文件
        </n-button>
      </template>
    </n-empty>

    <!-- Media Grid -->
    <div v-else class="media-grid">
      <div class="media-grid__info">
        <span class="media-grid__count">共 {{ pagination.total }} 个文件</span>
      </div>

      <div class="media-grid__items">
        <div
          v-for="file in files"
          :key="file.id"
          class="media-card"
        >
          <!-- Thumbnail / Preview -->
          <div
            class="media-card__preview"
            @click="openPreview(file)"
          >
            <img
              v-if="isImage(file.mime_type)"
              :src="file.url"
              :alt="file.original_name"
              class="media-card__image"
              loading="lazy"
            />
            <div v-else-if="isVideo(file.mime_type)" class="media-card__video-icon">
              <span class="media-card__video-symbol">▶</span>
              <span class="media-card__video-label">视频</span>
            </div>
            <div v-else class="media-card__generic-icon">
              <span>📄</span>
            </div>
          </div>

          <!-- File Info -->
          <div class="media-card__info">
            <div class="media-card__name" :title="file.original_name">
              {{ file.original_name }}
            </div>
            <div class="media-card__meta">
              <n-tag size="tiny" :bordered="false" type="info">
                {{ file.mime_type.split('/')[1].toUpperCase() }}
              </n-tag>
              <span class="media-card__size">{{ formatFileSize(file.size_bytes) }}</span>
            </div>
            <div class="media-card__date">{{ formatDate(file.created_at) }}</div>
          </div>

          <!-- Actions -->
          <div class="media-card__actions">
            <n-popconfirm
              @positive-click="handleDelete(file)"
              positive-text="确认删除"
              negative-text="取消"
            >
              <template #trigger>
                <n-button size="tiny" type="error" quaternary @click.stop>
                  删除
                </n-button>
              </template>
              <div class="delete-confirm">
                <p class="delete-confirm__title">确认删除此文件？</p>
                <p class="delete-confirm__desc">此操作不可撤销。</p>
                <p class="delete-confirm__file">{{ file.original_name }}</p>
              </div>
            </n-popconfirm>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="media-pagination">
        <n-pagination
          :page="pagination.page"
          :page-count="totalPages"
          :page-slot="7"
          @update:page="handlePageChange"
        />
      </div>
    </div>

    <!-- Preview Modal -->
    <div
      v-if="previewFile"
      class="media-preview-overlay"
      @click.self="closePreview"
    >
      <div class="media-preview">
        <div class="media-preview__header">
          <span class="media-preview__title">{{ previewFile.original_name }}</span>
          <n-button quaternary circle size="small" @click="closePreview">
            ✕
          </n-button>
        </div>
        <div class="media-preview__body">
          <img
            v-if="isImage(previewFile.mime_type)"
            :src="previewFile.url"
            :alt="previewFile.original_name"
            class="media-preview__image"
          />
          <video
            v-else-if="isVideo(previewFile.mime_type)"
            :src="previewFile.url"
            controls
            class="media-preview__video"
          />
        </div>
        <div class="media-preview__meta">
          <span>{{ previewFile.mime_type }}</span>
          <span>{{ formatFileSize(previewFile.size_bytes) }}</span>
          <span>{{ formatDate(previewFile.created_at) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* --- Page Layout --- */
.media-page {
  max-width: 960px;
  margin: 0 auto;
  padding: var(--space-8) var(--space-4);
  animation: fadeIn var(--transition-slow) ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* --- Header --- */
.media-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
  gap: var(--space-4);
}

.media-header__info {
  flex: 1;
}

.media-title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.media-subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Hidden Input --- */
.media-hidden-input {
  display: none;
}

/* --- Alert --- */
.media-alert {
  margin-bottom: var(--space-6);
  border-radius: var(--radius-md);
}

/* --- Drop Zone --- */
.media-dropzone {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 160px;
  margin-bottom: var(--space-8);
  border: 2px dashed var(--color-border);
  border-radius: var(--radius-lg);
  background: var(--color-bg-sunken);
  cursor: pointer;
  transition: border-color var(--transition-fast), background-color var(--transition-fast);
}

.media-dropzone:hover {
  border-color: var(--color-accent);
  background: var(--color-accent-light);
}

.media-dropzone--active {
  border-color: var(--color-accent);
  background: var(--color-accent-light);
  border-style: solid;
}

.media-dropzone__content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4);
}

.media-dropzone__icon {
  font-size: var(--text-3xl);
  line-height: 1;
  color: var(--color-accent);
  opacity: 0.7;
}

.media-dropzone__text {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  font-weight: 500;
}

.media-dropzone__hint {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

/* --- Loading --- */
.media-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  padding: var(--space-16) 0;
  color: var(--color-text-secondary);
}

/* --- Empty --- */
.media-empty {
  padding: var(--space-16) 0;
}

/* --- Grid --- */
.media-grid__info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.media-grid__count {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

.media-grid__items {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-8);
}

/* --- Media Card --- */
.media-card {
  display: flex;
  flex-direction: column;
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  overflow: hidden;
  box-shadow: var(--shadow-sm);
  transition: box-shadow var(--transition-base), transform var(--transition-base);
}

.media-card:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.media-card__preview {
  position: relative;
  aspect-ratio: 4 / 3;
  overflow: hidden;
  cursor: pointer;
  background: var(--color-bg-sunken);
  display: flex;
  align-items: center;
  justify-content: center;
}

.media-card__image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-base);
}

.media-card:hover .media-card__image {
  transform: scale(1.05);
}

.media-card__video-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  color: var(--color-text-secondary);
}

.media-card__video-symbol {
  font-size: var(--text-3xl);
  line-height: 1;
  color: var(--color-accent);
}

.media-card__video-label {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.media-card__generic-icon {
  font-size: var(--text-3xl);
  line-height: 1;
}

.media-card__info {
  padding: var(--space-3);
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.media-card__name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.media-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.media-card__size {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.media-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.media-card__actions {
  padding: 0 var(--space-3) var(--space-3);
  display: flex;
  justify-content: flex-end;
}

/* --- Pagination --- */
.media-pagination {
  display: flex;
  justify-content: center;
  padding: var(--space-4) 0;
}

/* --- Delete Confirm --- */
.delete-confirm {
  max-width: 260px;
}

.delete-confirm__title {
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
  font-size: var(--text-sm);
}

.delete-confirm__desc {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-2);
  line-height: 1.5;
}

.delete-confirm__file {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  background: var(--color-bg-sunken);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  word-break: break-all;
}

/* --- Preview Modal --- */
.media-preview-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.media-preview {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-xl);
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-lg);
}

.media-preview__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.media-preview__title {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.media-preview__body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
  min-height: 300px;
  max-height: calc(90vh - 140px);
  overflow: auto;
}

.media-preview__image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: var(--radius-md);
}

.media-preview__video {
  max-width: 100%;
  max-height: 100%;
  border-radius: var(--radius-md);
}

.media-preview__meta {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-3) var(--space-6);
  border-top: 1px solid var(--color-border);
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

/* --- Responsive --- */
@media (max-width: 768px) {
  .media-page {
    padding: var(--space-4) var(--space-3);
  }

  .media-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .media-title {
    font-size: var(--text-3xl);
  }

  .media-grid__items {
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: var(--space-3);
  }

  .media-preview {
    width: 95%;
  }
}
</style>
