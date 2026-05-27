<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NNotificationProvider } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import AppLayout from './components/Layout.vue'

// Naive UI 主题覆盖，匹配我们的设计系统
const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#C45D3E',
    primaryColorHover: '#A94E34',
    primaryColorPressed: '#A94E34',
    primaryColorSuppl: '#F5E6E0',
    bodyColor: '#FAFAF7',
    cardColor: '#FFFFFF',
    modalColor: '#FFFFFF',
    popoverColor: '#FFFFFF',
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
}
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <n-message-provider>
      <n-notification-provider>
        <app-layout />
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
</template>
