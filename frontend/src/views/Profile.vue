<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  NCard,
  NInput,
  NButton,
  NAlert,
  NTag,
  useMessage,
} from 'naive-ui'
import { useAuth } from '../composables/useAuth'

const message = useMessage()
const { user, loading, error, fetchMe, updateProfile, changePassword } = useAuth()

// ── Profile form ────────────────────────────────────────

const email = ref('')
const name = ref('')

// ── Password form ───────────────────────────────────────

const currentPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')

onMounted(async () => {
  if (!user.value) {
    await fetchMe()
  }
  if (user.value) {
    email.value = user.value.email
    name.value = user.value.name
  }
})

async function handleUpdateProfile() {
  if (!email.value.trim() || !name.value.trim()) {
    message.warning('请填写邮箱和昵称')
    return
  }

  try {
    await updateProfile(email.value.trim(), name.value.trim())
    message.success('个人信息已更新')
  } catch {
    // error is already set in composable
  }
}

async function handleChangePassword() {
  if (!currentPassword.value || !newPassword.value || !confirmPassword.value) {
    message.warning('请填写所有密码字段')
    return
  }

  if (newPassword.value !== confirmPassword.value) {
    message.warning('两次输入的新密码不一致')
    return
  }

  if (newPassword.value.length < 6) {
    message.warning('新密码长度至少为 6 位')
    return
  }

  try {
    await changePassword(currentPassword.value, newPassword.value)
    message.success('密码已修改')
    currentPassword.value = ''
    newPassword.value = ''
    confirmPassword.value = ''
  } catch {
    // error is already set in composable
  }
}

function getRoleLabel(role: string): string {
  const labels: Record<string, string> = {
    admin: '管理员',
    user: '普通用户',
    editor: '编辑',
  }
  return labels[role] || role
}
</script>

<template>
  <div class="profile-page">
    <h1 class="profile-page__title">个人中心</h1>
    <p class="profile-page__subtitle">管理你的账户信息</p>

    <n-alert
      v-if="error"
      type="error"
      :bordered="false"
      closable
      class="profile-page__alert"
      @close="error = null"
    >
      {{ error }}
    </n-alert>

    <!-- Current Info -->
    <n-card class="profile-page__card" :bordered="true">
      <template #header>
        <span class="profile-page__card-title">当前信息</span>
      </template>

      <div v-if="user" class="profile-page__info">
        <div class="profile-page__info-row">
          <span class="profile-page__info-label">邮箱</span>
          <span class="profile-page__info-value">{{ user.email }}</span>
        </div>
        <div class="profile-page__info-row">
          <span class="profile-page__info-label">昵称</span>
          <span class="profile-page__info-value">{{ user.name }}</span>
        </div>
        <div class="profile-page__info-row">
          <span class="profile-page__info-label">角色</span>
          <n-tag :type="user.role === 'admin' ? 'warning' : 'default'" size="small">
            {{ getRoleLabel(user.role) }}
          </n-tag>
        </div>
      </div>
      <div v-else class="profile-page__info-empty">
        加载中…
      </div>
    </n-card>

    <!-- Edit Profile -->
    <n-card class="profile-page__card" :bordered="true">
      <template #header>
        <span class="profile-page__card-title">编辑信息</span>
      </template>

      <form class="profile-page__form" @submit.prevent="handleUpdateProfile">
        <div class="profile-page__field">
          <label class="profile-page__label" for="profile-email">邮箱</label>
          <n-input
            id="profile-email"
            v-model:value="email"
            type="text"
            placeholder="name@example.com"
            size="large"
            :disabled="loading"
          />
        </div>

        <div class="profile-page__field">
          <label class="profile-page__label" for="profile-name">昵称</label>
          <n-input
            id="profile-name"
            v-model:value="name"
            placeholder="你的昵称"
            size="large"
            :disabled="loading"
          />
        </div>

        <n-button
          type="primary"
          size="large"
          :loading="loading"
          attr-type="submit"
          class="profile-page__submit"
        >
          保存修改
        </n-button>
      </form>
    </n-card>

    <!-- Change Password -->
    <n-card class="profile-page__card" :bordered="true">
      <template #header>
        <span class="profile-page__card-title">修改密码</span>
      </template>

      <form class="profile-page__form" @submit.prevent="handleChangePassword">
        <div class="profile-page__field">
          <label class="profile-page__label" for="current-password">当前密码</label>
          <n-input
            id="current-password"
            v-model:value="currentPassword"
            type="password"
            show-password-on="click"
            placeholder="输入当前密码"
            size="large"
            :disabled="loading"
          />
        </div>

        <div class="profile-page__field">
          <label class="profile-page__label" for="new-password">新密码</label>
          <n-input
            id="new-password"
            v-model:value="newPassword"
            type="password"
            show-password-on="click"
            placeholder="输入新密码"
            size="large"
            :disabled="loading"
          />
        </div>

        <div class="profile-page__field">
          <label class="profile-page__label" for="confirm-password">确认新密码</label>
          <n-input
            id="confirm-password"
            v-model:value="confirmPassword"
            type="password"
            show-password-on="click"
            placeholder="再次输入新密码"
            size="large"
            :disabled="loading"
          />
        </div>

        <n-button
          type="primary"
          size="large"
          :loading="loading"
          attr-type="submit"
          class="profile-page__submit"
        >
          修改密码
        </n-button>
      </form>
    </n-card>
  </div>
</template>

<style scoped>
.profile-page {
  max-width: 560px;
  margin: 0 auto;
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

.profile-page__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.profile-page__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  margin-bottom: var(--space-8);
}

.profile-page__alert {
  margin-bottom: var(--space-5);
}

.profile-page__card {
  background: var(--color-bg-elevated);
  border-color: var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
  margin-bottom: var(--space-6);
}

.profile-page__card-title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
}

.profile-page__info {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.profile-page__info-row {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.profile-page__info-label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-tertiary);
  min-width: 48px;
}

.profile-page__info-value {
  font-size: var(--text-base);
  color: var(--color-text-primary);
}

.profile-page__info-empty {
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

.profile-page__form {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.profile-page__field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.profile-page__label {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
}

.profile-page__submit {
  margin-top: var(--space-2);
  font-weight: 600;
  align-self: flex-start;
}

@media (max-width: 767px) {
  .profile-page {
    padding: var(--space-4);
  }

  .profile-page__title {
    font-size: var(--text-3xl);
  }
}
</style>
