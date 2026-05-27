<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  NButton,
  NCard,
  NInput,
  NSpin,
  NEmpty,
  NAlert,
  NPopconfirm,
  useMessage,
} from 'naive-ui'
import api from '../lib/api'
import { useAuth } from '../composables/useAuth'

const message = useMessage()
const { isAuthenticated, setAdminKey } = useAuth()

// --- Types ---
interface BackupInfo {
  filename: string
  size_bytes: number
  created_at: string
}

// --- State ---
const backups = ref<BackupInfo[]>([])
const total = ref(0)
const isLoading = ref(false)
const isCreating = ref(false)
const restoringFile = ref<string | null>(null)
const error = ref<string | null>(null)

// Auth dialog
const showAuthDialog = ref(false)
const adminKeyInput = ref('')

// --- Computed ---
const hasBackups = computed(() => backups.value.length > 0)

// --- Helpers ---
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const k = 1024
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${units[i]}`
}

function formatDateTime(isoStr: string): string {
  try {
    const date = new Date(isoStr)
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  } catch {
    return isoStr
  }
}

// --- API ---
async function fetchBackups() {
  if (!isAuthenticated.value) {
    showAuthDialog.value = true
    return
  }

  isLoading.value = true
  error.value = null

  try {
    const { data } = await api.get('/api/v1/backup/list')
    backups.value = data.backups || []
    total.value = data.total || 0
  } catch (err: any) {
    const status = err.response?.status
    if (status === 401) {
      showAuthDialog.value = true
      error.value = '认证失败，请检查管理密钥'
    } else {
      error.value = err.response?.data?.message || '获取备份列表失败'
    }
  } finally {
    isLoading.value = false
  }
}

async function createBackup() {
  if (!isAuthenticated.value) {
    showAuthDialog.value = true
    return
  }

  isCreating.value = true
  error.value = null

  try {
    await api.post('/api/v1/backup')
    message.success('备份创建成功')
    await fetchBackups()
  } catch (err: any) {
    const status = err.response?.status
    if (status === 401) {
      showAuthDialog.value = true
      error.value = '认证失败，请检查管理密钥'
    } else {
      const msg = err.response?.data?.message || '备份创建失败'
      error.value = msg
      message.error(msg)
    }
  } finally {
    isCreating.value = false
  }
}

async function restoreBackup(filename: string) {
  if (!isAuthenticated.value) {
    showAuthDialog.value = true
    return
  }

  restoringFile.value = filename
  error.value = null

  try {
    await api.post('/api/v1/backup/restore', { filename })
    message.success('数据库恢复成功')
  } catch (err: any) {
    const status = err.response?.status
    if (status === 401) {
      showAuthDialog.value = true
      error.value = '认证失败，请检查管理密钥'
    } else {
      const msg = err.response?.data?.message || '恢复失败'
      error.value = msg
      message.error(msg)
    }
  } finally {
    restoringFile.value = null
  }
}

// --- Auth ---
function handleAuthSubmit() {
  if (!adminKeyInput.value.trim()) {
    message.warning('请输入管理密钥')
    return
  }
  setAdminKey(adminKeyInput.value.trim())
  showAuthDialog.value = false
  adminKeyInput.value = ''
  message.success('认证成功')
  fetchBackups()
}

// --- Lifecycle ---
onMounted(() => {
  if (isAuthenticated.value) {
    fetchBackups()
  }
})
</script>

<template>
  <div class="backup-page">
    <!-- Header -->
    <header class="backup-header">
      <div class="backup-header__info">
        <h2 class="backup-title">数据备份</h2>
        <p class="backup-subtitle">管理数据库备份，手动创建或恢复备份</p>
      </div>
      <n-button
        type="primary"
        :loading="isCreating"
        @click="createBackup"
      >
        {{ isCreating ? '创建中...' : '创建备份' }}
      </n-button>
    </header>

    <!-- Error Alert -->
    <n-alert
      v-if="error"
      type="error"
      :bordered="false"
      closable
      @close="error = null"
      class="backup-alert"
    >
      {{ error }}
    </n-alert>

    <!-- Loading State -->
    <div v-if="isLoading" class="backup-loading">
      <n-spin size="medium" />
      <span>加载中...</span>
    </div>

    <!-- Empty State -->
    <n-empty
      v-else-if="!hasBackups && !isLoading"
      description="暂无备份记录"
      class="backup-empty"
    >
      <template #extra>
        <n-button type="primary" @click="createBackup" :loading="isCreating">
          创建第一个备份
        </n-button>
      </template>
    </n-empty>

    <!-- Backup List -->
    <div v-else class="backup-list">
      <div class="backup-list__header">
        <span class="backup-list__count">共 {{ total }} 个备份</span>
      </div>

      <div class="backup-list__grid">
        <n-card
          v-for="backup in backups"
          :key="backup.filename"
          class="backup-card"
          :bordered="false"
        >
          <div class="backup-card__content">
            <div class="backup-card__info">
              <div class="backup-card__filename">{{ backup.filename }}</div>
              <div class="backup-card__meta">
                <span class="backup-card__size">{{ formatFileSize(backup.size_bytes) }}</span>
                <span class="backup-card__divider">·</span>
                <span class="backup-card__time">{{ formatDateTime(backup.created_at) }}</span>
              </div>
            </div>
            <div class="backup-card__actions">
              <n-popconfirm
                @positive-click="restoreBackup(backup.filename)"
                positive-text="确认恢复"
                negative-text="取消"
              >
                <template #trigger>
                  <n-button
                    size="small"
                    :loading="restoringFile === backup.filename"
                    :disabled="restoringFile !== null"
                  >
                    {{ restoringFile === backup.filename ? '恢复中...' : '恢复' }}
                  </n-button>
                </template>
                <div class="restore-confirm">
                  <p class="restore-confirm__title">确认恢复此备份？</p>
                  <p class="restore-confirm__desc">
                    此操作将覆盖当前数据库数据，且不可撤销。
                  </p>
                  <p class="restore-confirm__file">{{ backup.filename }}</p>
                </div>
              </n-popconfirm>
            </div>
          </div>
        </n-card>
      </div>
    </div>

    <!-- Auth Dialog -->
    <div v-if="showAuthDialog" class="auth-overlay" @click.self="showAuthDialog = false">
      <div class="auth-dialog">
        <h3 class="auth-dialog__title">管理员认证</h3>
        <p class="auth-dialog__desc">
          请输入管理密钥以访问备份管理
        </p>
        <div class="auth-dialog__field">
          <label class="auth-dialog__label" for="admin-key">管理密钥</label>
          <n-input
            id="admin-key"
            v-model:value="adminKeyInput"
            type="password"
            placeholder="请输入 ADMIN_KEY"
            @keyup.enter="handleAuthSubmit"
          />
        </div>
        <div class="auth-dialog__actions">
          <n-button @click="showAuthDialog = false">取消</n-button>
          <n-button type="primary" @click="handleAuthSubmit">确认</n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* --- Page Layout --- */
.backup-page {
  max-width: 720px;
  margin: 0 auto;
  padding: var(--space-8) var(--space-4);
}

/* --- Header --- */
.backup-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--space-8);
  gap: var(--space-4);
}

.backup-header__info {
  flex: 1;
}

.backup-title {
  font-family: var(--font-display);
  font-size: var(--text-3xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.backup-subtitle {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  font-weight: 400;
}

/* --- Alert --- */
.backup-alert {
  margin-bottom: var(--space-6);
  border-radius: var(--radius-md);
}

/* --- Loading --- */
.backup-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-3);
  padding: var(--space-16) 0;
  color: var(--color-text-secondary);
}

/* --- Empty --- */
.backup-empty {
  padding: var(--space-16) 0;
}

/* --- List --- */
.backup-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.backup-list__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.backup-list__count {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

.backup-list__grid {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

/* --- Backup Card --- */
.backup-card {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  transition: box-shadow var(--transition-base), transform var(--transition-base);
}

.backup-card:hover {
  box-shadow: var(--shadow-md);
}

.backup-card__content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
}

.backup-card__info {
  flex: 1;
  min-width: 0;
}

.backup-card__filename {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
  margin-bottom: var(--space-1);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.backup-card__meta {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.backup-card__divider {
  opacity: 0.4;
}

.backup-card__actions {
  flex-shrink: 0;
}

/* --- Restore Confirm --- */
.restore-confirm {
  max-width: 280px;
}

.restore-confirm__title {
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
  font-size: var(--text-sm);
}

.restore-confirm__desc {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-2);
  line-height: 1.5;
}

.restore-confirm__file {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  background: var(--color-bg-sunken);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  word-break: break-all;
}

/* --- Auth Dialog --- */
.auth-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.auth-dialog {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  padding: var(--space-8);
  width: 90%;
  max-width: 400px;
  box-shadow: var(--shadow-lg);
}

.auth-dialog__title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-2);
}

.auth-dialog__desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-6);
}

.auth-dialog__field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-bottom: var(--space-6);
}

.auth-dialog__label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.auth-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

/* --- Responsive --- */
@media (max-width: 768px) {
  .backup-page {
    padding: var(--space-4) var(--space-3);
  }

  .backup-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .backup-title {
    font-size: var(--text-2xl);
  }

  .backup-card__content {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }

  .backup-card__actions {
    width: 100%;
  }

  .backup-card__actions .n-button {
    width: 100%;
  }
}
</style>
