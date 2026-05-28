<script setup lang="ts">
import { h, ref, computed, watch, inject, onMounted, onUnmounted, type Ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  NLayout,
  NLayoutSider,
  NLayoutContent,
  NMenu,
  NButton,
  darkTheme,
} from 'naive-ui'
import { useResponsive } from '../composables/useResponsive'
import { useAppStore } from '../stores/app'
import { useAuth } from '../composables/useAuth'

const router = useRouter()
const route = useRoute()
const appStore = useAppStore()
const { isDesktop } = useResponsive()
const { user, isAuthenticated, logout } = useAuth()

function handleLogout() {
  logout()
  router.push('/')
}

// Injected from App.vue
const theme = inject<Ref<typeof darkTheme | null>>('theme')
const toggleTheme = inject<() => void>('toggleTheme')
const isDark = computed(() => theme?.value === darkTheme)

// 渲染图标辅助函数
function renderIcon(icon: string) {
  return () => h('span', { style: 'font-size: 18px; line-height: 1;' }, icon)
}

// 带图标的菜单选项（按角色动态显示）
const menuOptions = computed(() => {
  const items = []

  // 所有用户可见
  items.push({ label: '首页', key: '/', icon: renderIcon('⌂') })
  items.push({ label: '文章列表', key: '/posts', icon: renderIcon('☰') })

  if (!isAuthenticated.value) {
    // 访客
    items.push({ label: '登录', key: '/login', icon: renderIcon('→') })
  } else if (user.value?.role === 'admin' || !user.value) {
    // 管理员（含旧 ADMIN_KEY 登录无 user 信息的情况）
    items.push(
      { label: '发布文章', key: '/publish', icon: renderIcon('✎') },
      { label: '草稿箱', key: '/drafts', icon: renderIcon('📝') },
      { label: '导入文章', key: '/import', icon: renderIcon('↓') },
      { label: '数据备份', key: '/backup', icon: renderIcon('⤓') },
      { label: '评论管理', key: '/comments', icon: renderIcon('💬') },
      { label: '媒体管理', key: '/media', icon: renderIcon('🖼') },
      { label: 'TODO 管理', key: '/todos', icon: renderIcon('✓') },
      { label: '标签管理', key: '/tags', icon: renderIcon('🏷') },
      { label: '回收站', key: '/trash', icon: renderIcon('🗑') },
    )
  } else {
    // 普通用户
    items.push(
      { label: '我的订阅', key: '/my/subscriptions', icon: renderIcon('🔔') },
      { label: '我的 PR', key: '/my/pulls', icon: renderIcon('⑂') },
      { label: '我的评论', key: '/my/comments', icon: renderIcon('💬') },
      { label: '个人中心', key: '/profile', icon: renderIcon('👤') },
    )
  }

  return items
})

// ── 移动端底部导航：按角色动态生成 ─────────────────────────

interface MobileNavItem {
  key: string
  label: string
  icon: string
  children?: { key: string; label: string; icon: string }[]
}

const mobileNavItems = computed<MobileNavItem[]>(() => {
  // 访客
  if (!isAuthenticated.value) {
    return [
      { key: '/', label: '首页', icon: '⌂' },
      { key: '/posts', label: '文章', icon: '☰' },
      { key: '/login', label: '登录', icon: '→' },
    ]
  }

  // 管理员（含旧 ADMIN_KEY 登录无 user 信息的情况）
  if (user.value?.role === 'admin' || !user.value) {
    return [
      { key: '/', label: '首页', icon: '⌂' },
      { key: '/publish', label: '发布', icon: '✎' },
      {
        key: 'admin',
        label: '管理',
        icon: '⚙',
        children: [
          { key: '/drafts', label: '草稿箱', icon: '📝' },
          { key: '/comments', label: '评论', icon: '💬' },
          { key: '/media', label: '媒体', icon: '🖼' },
          { key: '/tags', label: '标签', icon: '🏷' },
          { key: '/todos', label: 'TODO', icon: '✓' },
          { key: '/backup', label: '备份', icon: '⤓' },
          { key: '/import', label: '导入', icon: '↓' },
        ],
      },
    ]
  }

  // 普通用户
  return [
    { key: '/', label: '首页', icon: '⌂' },
    { key: '/posts', label: '文章', icon: '☰' },
    {
      key: 'my',
      label: '我的',
      icon: '👤',
      children: [
        { key: '/profile', label: '个人资料', icon: '✎' },
      ],
    },
  ]
})

// 子菜单状态管理
const activeSubmenu = ref<string | null>(null)

function toggleSubmenu(key: string) {
  activeSubmenu.value = activeSubmenu.value === key ? null : key
}

function closeSubmenu() {
  activeSubmenu.value = null
}

// 点击外部关闭子菜单
function handleOutsideClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (!target.closest('.bottom-nav__item--has-children')) {
    closeSubmenu()
  }
}

onMounted(() => {
  document.addEventListener('click', handleOutsideClick)
})

onUnmounted(() => {
  document.removeEventListener('click', handleOutsideClick)
})

// 处理子菜单项点击
function handleSubmenuItemClick(key: string) {
  appStore.setActiveRoute(key)
  router.push(key)
  closeSubmenu()
}

// 判断路由是否匹配某个子菜单
function isInSubmenu(item: MobileNavItem): boolean {
  if (!item.children) return false
  return item.children.some(
    (child) =>
      appStore.activeRoute === child.key ||
      appStore.activeRoute.startsWith(child.key + '/')
  )
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

// 侧边栏搜索
const sidebarSearchQuery = ref('')
function handleSidebarSearch() {
  const q = sidebarSearchQuery.value.trim()
  if (q) {
    router.push({ path: '/', query: { q } })
    sidebarSearchQuery.value = ''
  }
}
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
        <n-button
          quaternary
          circle
          size="small"
          class="theme-toggle"
          @click="toggleTheme"
        >
          {{ isDark ? '☀' : '☾' }}
        </n-button>
      </div>

      <!-- Sidebar Search (only when expanded) -->
      <div v-if="!appStore.sidebarCollapsed" class="sidebar-search">
        <n-input
          v-model:value="sidebarSearchQuery"
          placeholder="搜索文章…"
          size="small"
          clearable
          @keyup.enter="handleSidebarSearch"
        >
          <template #prefix>
            <span style="font-size: 14px; opacity: 0.6;">⌕</span>
          </template>
        </n-input>
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

      <!-- User Area (sidebar bottom) -->
      <div class="sidebar-user">
        <template v-if="isAuthenticated">
          <span v-if="user" class="user-name">{{ user.name }}</span>
          <n-button quaternary size="small" @click="handleLogout">登出</n-button>
        </template>
        <template v-else>
          <n-button
            quaternary
            size="small"
            @click="router.push('/login')"
          >
            登录
          </n-button>
        </template>
      </div>
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
      <div class="mobile-header__search">
        <n-input
          v-model:value="sidebarSearchQuery"
          placeholder="搜索…"
          size="small"
          clearable
          @keyup.enter="handleSidebarSearch"
        >
          <template #prefix>
            <span style="font-size: 14px; opacity: 0.6;">⌕</span>
          </template>
        </n-input>
      </div>
      <n-button
        quaternary
        circle
        size="small"
        class="theme-toggle"
        @click="toggleTheme"
      >
        {{ isDark ? '☀' : '☾' }}
      </n-button>
    </header>

    <!-- Main Content -->
    <n-layout-content :native-scrollbar="false">
      <div class="content-wrapper content-wrapper--mobile">
        <router-view />
      </div>
    </n-layout-content>

    <!-- Bottom Navigation Bar -->
    <nav class="bottom-nav">
      <template v-for="item in mobileNavItems" :key="item.key">
        <!-- 普通链接项 -->
        <router-link
          v-if="!item.children"
          :to="item.key"
          class="bottom-nav__item"
          :class="{ 'bottom-nav__item--active': appStore.activeRoute === item.key }"
          @click="appStore.setActiveRoute(item.key)"
        >
          <span class="bottom-nav__icon">{{ item.icon }}</span>
          <span class="bottom-nav__label">{{ item.label }}</span>
        </router-link>

        <!-- 带子菜单的项 -->
        <div
          v-else
          class="bottom-nav__item bottom-nav__item--has-children"
          :class="{
            'bottom-nav__item--active': isInSubmenu(item),
            'bottom-nav__item--expanded': activeSubmenu === item.key,
          }"
          @click.stop="toggleSubmenu(item.key)"
        >
          <span class="bottom-nav__icon">{{ item.icon }}</span>
          <span class="bottom-nav__label">{{ item.label }}</span>

          <!-- 子菜单弹出层 -->
          <Transition name="submenu">
            <div
              v-if="activeSubmenu === item.key"
              class="bottom-nav__submenu"
              @click.stop
            >
              <button
                v-for="child in item.children"
                :key="child.key"
                class="submenu-item"
                :class="{ 'submenu-item--active': appStore.activeRoute === child.key }"
                @click="handleSubmenuItemClick(child.key)"
              >
                <span class="submenu-item__icon">{{ child.icon }}</span>
                <span class="submenu-item__label">{{ child.label }}</span>
              </button>
            </div>
          </Transition>
        </div>
      </template>
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

.sidebar-search {
  padding: var(--space-2) var(--space-4) var(--space-3);
}

.sidebar-user {
  padding: var(--space-4);
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  margin-top: auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.user-name {
  color: rgba(255, 255, 255, 0.7);
  font-size: var(--text-sm);
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

/* Theme toggle button */
.theme-toggle {
  margin-left: auto;
  color: rgba(255, 255, 255, 0.6);
  font-size: 16px;
  flex-shrink: 0;
  transition: color var(--transition-fast);
}

.theme-toggle:hover {
  color: #ffffff;
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

.mobile-header__search {
  flex: 1;
  min-width: 0;
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

/* --- 子菜单触发项 --- */
.bottom-nav__item--has-children {
  cursor: pointer;
  position: relative;
}

/* --- 子菜单弹出层 --- */
.bottom-nav__submenu {
  position: absolute;
  bottom: calc(100% + var(--space-2));
  left: 50%;
  transform: translateX(-50%);
  min-width: 160px;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2);
  box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.15);
  z-index: 110;
}

.submenu-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  width: 100%;
  padding: var(--space-3) var(--space-4);
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  font-size: var(--text-sm);
  font-family: inherit;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background-color var(--transition-fast), color var(--transition-fast);
  -webkit-tap-highlight-color: transparent;
}

.submenu-item:active {
  background: var(--color-bg-sunken);
}

.submenu-item--active {
  color: var(--color-accent);
  background: var(--color-bg-sunken);
}

.submenu-item__icon {
  font-size: 16px;
  line-height: 1;
  flex-shrink: 0;
}

.submenu-item__label {
  white-space: nowrap;
}

/* 子菜单动画 */
.submenu-enter-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.submenu-leave-active {
  transition: opacity 0.1s ease, transform 0.1s ease;
}

.submenu-enter-from,
.submenu-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(4px);
}

.submenu-enter-to,
.submenu-leave-from {
  opacity: 1;
  transform: translateX(-50%) translateY(0);
}
</style>

<!-- Global dark mode overrides for Naive UI components -->
<style>
/* Force n-layout to use our background colors */
.n-layout {
  --n-color: var(--color-bg) !important;
  --n-color-embedded: var(--color-bg) !important;
}

.n-layout-content {
  --n-color: var(--color-bg) !important;
}

[data-theme="dark"] .sidebar-search .n-input {
  --n-color: rgba(255, 255, 255, 0.08);
  --n-color-focus: rgba(255, 255, 255, 0.12);
  --n-border: 1px solid rgba(255, 255, 255, 0.15);
  --n-text-color: rgba(255, 255, 255, 0.85);
  --n-placeholder-color: rgba(255, 255, 255, 0.35);
}

[data-theme="dark"] .n-layout-sider__trigger {
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.6);
}

[data-theme="dark"] .n-layout-sider__trigger:hover {
  background: rgba(255, 255, 255, 0.15);
  color: rgba(255, 255, 255, 0.9);
}
</style>
