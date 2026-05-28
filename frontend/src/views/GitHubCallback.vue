<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NSpin, NAlert } from 'naive-ui'
import { ref } from 'vue'
import { useAuth } from '../composables/useAuth'
import { useAppStore } from '../stores/app'

const router = useRouter()
const { setTokens, fetchMe, user } = useAuth()
const appStore = useAppStore()

const statusMessage = ref('正在处理 GitHub 登录…')
const errorMsg = ref<string | null>(null)

onMounted(async () => {
  const url = new URL(window.location.href)

  // Try query params first: ?access_token=...&refresh_token=...
  let accessToken = url.searchParams.get('access_token')
  let refreshToken = url.searchParams.get('refresh_token')

  // Fallback: check URL fragment (#access_token=...&refresh_token=...)
  if (!accessToken && url.hash) {
    const hashParams = new URLSearchParams(url.hash.substring(1))
    accessToken = hashParams.get('access_token')
    refreshToken = hashParams.get('refresh_token')
  }

  // Check for error from backend
  const errorParam = url.searchParams.get('error') ?? (url.hash
    ? new URLSearchParams(url.hash.substring(1)).get('error')
    : null)

  if (errorParam) {
    errorMsg.value = decodeURIComponent(errorParam)
    statusMessage.value = ''
    return
  }

  if (!accessToken || !refreshToken) {
    errorMsg.value = '未收到认证令牌，请重试'
    statusMessage.value = ''
    return
  }

  try {
    setTokens(accessToken, refreshToken)
    await fetchMe()

    if (user.value) {
      appStore.setUser(user.value)
    }

    router.push('/')
  } catch {
    errorMsg.value = '登录失败，请重试'
    statusMessage.value = ''
  }
})
</script>

<template>
  <div class="callback-page">
    <div class="callback-page__content">
      <template v-if="errorMsg">
        <n-alert type="error" :bordered="false" class="callback-page__alert">
          {{ errorMsg }}
        </n-alert>
      </template>
      <template v-else>
        <n-spin size="large" />
        <p class="callback-page__message">{{ statusMessage }}</p>
      </template>
    </div>
  </div>
</template>

<style scoped>
.callback-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - var(--header-height) - var(--space-16));
  padding: var(--space-6);
}

.callback-page__content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-4);
  max-width: 420px;
  width: 100%;
}

.callback-page__message {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  text-align: center;
}

.callback-page__alert {
  width: 100%;
}
</style>
