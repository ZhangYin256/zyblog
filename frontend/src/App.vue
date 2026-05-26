<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NNotificationProvider } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import AppLayout from './components/Layout.vue'

// Naive UI theme overrides to match our design system
const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: 'var(--color-accent)',
    primaryColorHover: 'var(--color-accent-hover)',
    primaryColorPressed: 'var(--color-accent-hover)',
    primaryColorSuppl: 'var(--color-accent-light)',
    bodyColor: 'var(--color-bg)',
    cardColor: 'var(--color-bg-elevated)',
    modalColor: 'var(--color-bg-elevated)',
    popoverColor: 'var(--color-bg-elevated)',
    borderRadius: '8px',
    borderRadiusSmall: '4px',
    fontFamily: 'var(--font-body)',
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
