<script setup lang="ts">
import { ref, provide, computed, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, NNotificationProvider, darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import AppLayout from './components/Layout.vue'

// Theme state: null = light, darkTheme = dark
const theme = ref<typeof darkTheme | null>(null)

// Sync data-theme attribute on <html> element
function syncThemeAttribute(dark: boolean) {
  document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light')
}

// Toggle theme and persist to localStorage
function toggleTheme() {
  theme.value = theme.value === null ? darkTheme : null
  localStorage.setItem('zyblog_theme', theme.value === null ? 'light' : 'dark')
  syncThemeAttribute(theme.value !== null)
}

// Provide theme + toggle to child components
provide('theme', theme)
provide('toggleTheme', toggleTheme)

// On mount: restore from localStorage or respect system preference
onMounted(() => {
  const saved = localStorage.getItem('zyblog_theme')
  if (saved === 'dark') {
    theme.value = darkTheme
  } else if (saved === 'light') {
    theme.value = null
  } else {
    // No saved preference — respect system
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    theme.value = prefersDark ? darkTheme : null
  }
  syncThemeAttribute(theme.value !== null)
})

// Naive UI 主题覆盖，匹配我们的设计系统（随明暗主题切换）
const isDark = computed(() => theme.value === darkTheme)

const themeOverrides = computed<GlobalThemeOverrides>(() => ({
  common: {
    primaryColor: '#C45D3E',
    primaryColorHover: '#A94E34',
    primaryColorPressed: '#A94E34',
    primaryColorSuppl: '#F5E6E0',
    bodyColor: isDark.value ? '#1A1A1A' : '#FAFAF7',
    cardColor: isDark.value ? '#242424' : '#FFFFFF',
    modalColor: isDark.value ? '#242424' : '#FFFFFF',
    popoverColor: isDark.value ? '#2A2A2A' : '#FFFFFF',
    borderRadius: '8px',
    borderRadiusSmall: '4px',
    fontFamily: "'Source Sans 3', 'Segoe UI', system-ui, -apple-system, sans-serif",
    fontSize: '1rem',
    fontSizeMini: '0.75rem',
    fontSizeTiny: '0.75rem',
    fontSizeSmall: '0.875rem',
    fontSizeMedium: '1rem',
    fontSizeLarge: '1.125rem',
    fontSizeHuge: '1.25rem',
    heightMedium: '40px',
    heightLarge: '48px',
  },
  Button: {
    borderRadiusMedium: '8px',
    borderRadiusLarge: '10px',
  },
  Input: {
    borderRadius: '8px',
  },
  Card: {
    borderRadius: '12px',
  },
  Tag: {
    borderRadius: '6px',
  },
  Menu: {
    itemTextColor: 'rgba(255, 255, 255, 0.7)',
    itemTextColorHover: '#ffffff',
    itemTextColorActive: '#ffffff',
    itemTextColorActiveHover: '#ffffff',
    itemTextColorChildActive: 'rgba(255, 255, 255, 0.9)',
    itemTextColorChildActiveHover: '#ffffff',
    itemIconColor: 'rgba(255, 255, 255, 0.5)',
    itemIconColorHover: 'rgba(255, 255, 255, 0.9)',
    itemIconColorActive: '#ffffff',
    itemIconColorActiveHover: '#ffffff',
    itemIconColorChildActive: 'rgba(255, 255, 255, 0.8)',
    itemIconColorChildActiveHover: '#ffffff',
    itemColorActive: 'rgba(196, 93, 62, 0.25)',
    itemColorActiveHover: 'rgba(196, 93, 62, 0.3)',
    itemColorHover: 'rgba(255, 255, 255, 0.06)',
    arrowColor: 'rgba(255, 255, 255, 0.3)',
    arrowColorHover: 'rgba(255, 255, 255, 0.6)',
    arrowColorActive: '#ffffff',
    arrowColorActiveHover: '#ffffff',
  },
}))
</script>

<template>
  <div style="min-height: 100vh">
    <n-config-provider :theme="theme" :theme-overrides="themeOverrides">
      <n-message-provider>
        <n-notification-provider>
          <app-layout />
        </n-notification-provider>
      </n-message-provider>
    </n-config-provider>
  </div>
</template>
