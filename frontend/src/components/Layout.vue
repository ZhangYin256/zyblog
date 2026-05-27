<script setup lang="ts">
import { h, computed, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu,
} from 'naive-ui'
import { useResponsive } from '../composables/useResponsive'
import { useAppStore } from '../stores/app'

const router = useRouter()
const route = useRoute()
const appStore = useAppStore()
const { isDesktop } = useResponsive()

// 渲染图标辅助函数
function renderIcon(icon: string) {
  return () => h('span', { style: 'font-size: 18px; line-height: 1;' }, icon)
}

// 带图标的菜单选项
const menuOptions = computed(() => [
  {
    label: '首页',
    key: '/',
    icon: renderIcon('⌂'),
  },
  {
    label: '发布文章',
    key: '/publish',
    icon: renderIcon('✎'),
  },
  {
    label: '文章列表',
    key: '/posts',
    icon: renderIcon('☰'),
  },
  {
    label: '导入文章',
    key: '/import',
    icon: renderIcon('↓'),
  },
])

// 移动端导航图标映射
const navIconMap: Record<string, string> = {
  '/': '⌂',
  '/publish': '✎',
  '/posts': '☰',
  '/import': '↓',
}

function getNavIcon(key: string): string {
  return navIconMap[key] || '•'
}

// 处理菜单点击
function handleMenuUpdate(key: string) {
  appStore.setActiveRoute(key)
  router.push(key)
}

// 同步路由与活动菜单
watch(
  () => route.path,
  (path) => {
    const matchedPath = menuOptions.value.find(
      (opt) => path === opt.key || path.startsWith(opt.key + '/')
    )
    if (matchedPath) {
      appStore.setActiveRoute(matchedPath.key)
    }
  },
  { immediate: true }
)

// 侧边栏尺寸
const siderWidth = 240
const collapsedWidth = 64
</script>

<template>
  <!-- PC Layout: Sidebar + Content -->
  <n-layout
    v-if="isDesktop"
    has-sider
    style="height: 100vh;"
    :sider-placement="'left'"
  >
    <!-- Sidebar -->
    <n-layout-sider
      bordered
      :width="siderWidth"
      :collapsed-width="collapsedWidth"
      :collapsed="appStore.sidebarCollapsed"
      show-trigger
      @collapse="appStore.sidebarCollapsed = true"
      @expand="appStore.sidebarCollapsed = false"
      :native-scrollbar="false"
      :style="{
        backgroundColor: 'var(--color-bg-sidebar)',
      }"
    >
      <!-- Logo / Brand -->
      <div class="sidebar-brand">
        <span class="brand-mark">Z</span>
        <span v-if="!appStore.sidebarCollapsed" class="brand-text">ZYBlog</span>
      </div>

      <!-- Navigation Menu -->
      <n-menu
        :value="appStore.activeRoute"
        :options="menuOptions"
        :collapsed="appStore.sidebarCollapsed"
        :collapsed-width="collapsedWidth"
        :collapsed-icon-size="20"
        :indent="24"
        @update:value="handleMenuUpdate"
      />
    </n-layout-sider>

    <!-- Main Content Area -->
    <n-layout-content
      :native-scrollbar="false"
      content-style="min-height: 100vh;"
    >
      <div class="content-wrapper">
        <router-view />
      </div>
    </n-layout-content>
  </n-layout>

  <!-- Mobile Layout: Single Column + Bottom Nav -->
  <n-layout v-else style="min-height: 100vh; padding-bottom: var(--bottom-nav-height);">
    <!-- Mobile Header -->
    <header class="mobile-header">
      <span class="brand-mark">Z</span>
      <span class="brand-text">ZYBlog</span>
    </header>

    <!-- Main Content -->
    <n-layout-content :native-scrollbar="false">
      <div class="content-wrapper content-wrapper--mobile">
        <router-view />
      </div>
    </n-layout-content>

    <!-- Bottom Navigation Bar -->
    <nav class="bottom-nav">
      <router-link
        v-for="item in menuOptions"
        :key="item.key"
        :to="item.key"
        class="bottom-nav__item"
        :class="{ 'bottom-nav__item--active': appStore.activeRoute === item.key }"
        @click="appStore.setActiveRoute(item.key)"
      >
        <span class="bottom-nav__icon">{{ getNavIcon(item.key) }}</span>
        <span class="bottom-nav__label">{{ item.label }}</span>
      </router-link>
    </nav>
  </n-layout>
</template>

<style scoped>
/* --- 侧边栏样式 --- */
.sidebar-brand {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-6) var(--space-6) var(--space-4);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  margin-bottom: var(--space-2);
}

.brand-mark {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: var(--color-accent);
  color: var(--color-text-inverse);
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 700;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.brand-text {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-inverse);
  white-space: nowrap;
  letter-spacing: -0.02em;
}

/* Naive UI 侧边栏菜单深度样式覆盖 */
:deep(.n-menu) {
  --n-item-text-color: rgba(255, 255, 255, 0.7);
  --n-item-text-color-hover: #ffffff;
  --n-item-text-color-active: #ffffff;
  --n-item-icon-color: rgba(255, 255, 255, 0.5);
  --n-item-icon-color-hover: rgba(255, 255, 255, 0.9);
  --n-item-icon-color-active: #ffffff;
  --n-item-color-active: rgba(196, 93, 62, 0.2);
  --n-item-color-hover: rgba(255, 255, 255, 0.06);
  --n-arrow-color: rgba(255, 255, 255, 0.3);
  --n-arrow-color-hover: rgba(255, 255, 255, 0.6);
  --n-arrow-color-active: #ffffff;
  padding: var(--space-2) 0;
}

:deep(.n-menu-item-content) {
  border-radius: var(--radius-md);
  margin: 2px var(--space-3);
  padding-left: var(--space-3) !important;
}

:deep(.n-menu-item-content--selected) {
  background: rgba(196, 93, 62, 0.25) !important;
}

:deep(.n-menu-item-content--selected::before) {
  content: '';
  position: absolute;
  left: 0;
  top: 20%;
  height: 60%;
  width: 3px;
  background: var(--color-accent);
  border-radius: 0 2px 2px 0;
}

/* --- 内容区域 --- */
.content-wrapper {
  max-width: var(--content-max-width);
  margin: 0 auto;
  padding: var(--space-8) var(--space-6);
}

.content-wrapper--mobile {
  padding: var(--space-4);
}

/* --- 移动端头部 --- */
.mobile-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4) var(--space-4);
  background: var(--color-bg-sidebar);
  position: sticky;
  top: 0;
  z-index: 100;
}

.mobile-header .brand-mark {
  width: 32px;
  height: 32px;
  font-size: var(--text-lg);
}

.mobile-header .brand-text {
  font-size: var(--text-lg);
}

/* --- 底部导航 --- */
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: var(--bottom-nav-height);
  background: var(--color-bg-elevated);
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-around;
  z-index: 100;
  padding: 0 var(--space-2);
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.06);
}

.bottom-nav__item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  text-decoration: none;
  color: var(--color-text-tertiary);
  transition: color var(--transition-fast), background-color var(--transition-fast);
  min-width: 56px;
  -webkit-tap-highlight-color: transparent;
}

.bottom-nav__item:active {
  background: var(--color-bg-sunken);
}

.bottom-nav__item--active {
  color: var(--color-accent);
}

.bottom-nav__icon {
  font-size: 20px;
  line-height: 1;
}

.bottom-nav__label {
  font-size: var(--text-xs);
  font-weight: 500;
  line-height: 1.2;
}
</style>
