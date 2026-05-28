<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  NCard,
  NInput,
  NButton,
  NAlert,
  NDivider,
  useMessage,
} from 'naive-ui'
import { useAuth } from '../composables/useAuth'
import { useAppStore } from '../stores/app'

const router = useRouter()
const message = useMessage()
const { login, register, getGitHubOAuthUrl, loading, error, user } = useAuth()
const appStore = useAppStore()

const isRegisterMode = ref(false)
const email = ref('')
const password = ref('')
const name = ref('')
const githubAvailable = ref(false)

onMounted(async () => {
  try {
    const resp = await fetch('/api/v1/auth/github', { redirect: 'manual' })
    // 302 redirect → type is 'opaqueredirect' (GitHub OAuth configured)
    // 500 error → type is 'basic' (not configured)
    githubAvailable.value = resp.type === 'opaqueredirect' || (resp.ok && resp.status < 400)
  } catch {
    githubAvailable.value = false
  }
})

async function handleSubmit() {
  if (!email.value || !password.value) {
    message.warning('请填写邮箱和密码')
    return
  }

  if (isRegisterMode.value && !name.value) {
    message.warning('请填写昵称')
    return
  }

  try {
    if (isRegisterMode.value) {
      await register(email.value, password.value, name.value)
    } else {
      await login(email.value, password.value)
    }

    if (user.value) {
      appStore.setUser(user.value)
    }

    message.success(isRegisterMode.value ? '注册成功' : '登录成功')
    router.push('/')
  } catch {
    // error is already set in composable
  }
}

function toggleMode() {
  isRegisterMode.value = !isRegisterMode.value
  error.value = null
}

function goToGitHub() {
  window.location.href = getGitHubOAuthUrl()
}
</script>

<template>
  <div class="login-page">
    <div class="login-page__wrapper">
      <h1 class="login-page__title">{{ isRegisterMode ? '注册' : '登录' }}</h1>
      <p class="login-page__subtitle">
        {{ isRegisterMode ? '创建一个新账户' : '欢迎回来' }}
      </p>

      <n-card class="login-page__card" :bordered="true">
        <n-alert
          v-if="error"
          type="error"
          :bordered="false"
          closable
          class="login-page__alert"
          @close="error = null"
        >
          {{ error }}
        </n-alert>

        <form class="login-page__form" @submit.prevent="handleSubmit">
          <div v-if="isRegisterMode" class="login-page__field">
            <label class="login-page__label" for="name">昵称</label>
            <n-input
              id="name"
              v-model:value="name"
              placeholder="你的昵称"
              size="large"
              :disabled="loading"
            />
          </div>

          <div class="login-page__field">
            <label class="login-page__label" for="email">邮箱</label>
            <n-input
              id="email"
              v-model:value="email"
              type="text"
              placeholder="name@example.com"
              size="large"
              :disabled="loading"
            />
          </div>

          <div class="login-page__field">
            <label class="login-page__label" for="password">密码</label>
            <n-input
              id="password"
              v-model:value="password"
              type="password"
              show-password-on="click"
              placeholder="输入密码"
              size="large"
              :disabled="loading"
            />
          </div>

          <n-button
            type="primary"
            block
            size="large"
            :loading="loading"
            attr-type="submit"
            class="login-page__submit"
          >
            {{ isRegisterMode ? '注册' : '登录' }}
          </n-button>
        </form>

        <template v-if="githubAvailable">
          <n-divider class="login-page__divider">或</n-divider>

          <n-button
            block
            size="large"
            quaternary
            class="login-page__github"
            @click="goToGitHub"
          >
            <template #icon>
              <span class="login-page__github-icon">
                <svg height="20" width="20" viewBox="0 0 16 16" fill="currentColor">
                  <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
                </svg>
              </span>
            </template>
            使用 GitHub 登录
          </n-button>
        </template>

        <div class="login-page__toggle">
          <span class="login-page__toggle-text">
            {{ isRegisterMode ? '已有账户？' : '没有账户？' }}
          </span>
          <n-button text type="primary" @click="toggleMode">
            {{ isRegisterMode ? '去登录' : '去注册' }}
          </n-button>
        </div>
      </n-card>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - var(--header-height) - var(--space-16));
  padding: var(--space-6);
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

.login-page__wrapper {
  width: 100%;
  max-width: 420px;
}

.login-page__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  text-align: center;
  margin-bottom: var(--space-2);
}

.login-page__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  text-align: center;
  margin-bottom: var(--space-8);
}

.login-page__card {
  background: var(--color-bg-elevated);
  border-color: var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
}

.login-page__alert {
  margin-bottom: var(--space-5);
}

.login-page__form {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.login-page__field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.login-page__label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
}

.login-page__submit {
  margin-top: var(--space-2);
  font-weight: 600;
}

.login-page__divider {
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

.login-page__github {
  color: var(--color-text-primary);
  border-color: var(--color-border);
  font-weight: 500;
}

.login-page__github:hover {
  border-color: var(--color-text-tertiary);
}

.login-page__github-icon {
  display: inline-flex;
  align-items: center;
  margin-right: var(--space-2);
}

.login-page__toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  margin-top: var(--space-5);
}

.login-page__toggle-text {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

@media (max-width: 767px) {
  .login-page {
    padding: var(--space-4);
    align-items: flex-start;
    padding-top: var(--space-12);
  }

  .login-page__title {
    font-size: var(--text-3xl);
  }
}
</style>
