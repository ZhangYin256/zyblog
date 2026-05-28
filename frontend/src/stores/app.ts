import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User } from '../composables/useAuth'

/**
 * 全局应用状态存储
 * 管理侧边栏可见性、主题偏好、UI 状态和用户信息
 */
export const useAppStore = defineStore('app', () => {
  // 侧边栏状态（仅 PC）
  const sidebarCollapsed = ref(false)

  // 当前活动路由键，用于菜单高亮
  const activeRoute = ref('/')

  // 移动端底部导航可见性
  const bottomNavVisible = ref(true)

  // 用户信息
  const currentUser = ref<User | null>(null)
  const isLoggedIn = computed(() => currentUser.value !== null)

  // 操作 — UI
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  function setActiveRoute(route: string) {
    activeRoute.value = route
  }

  function setBottomNavVisible(visible: boolean) {
    bottomNavVisible.value = visible
  }

  // 操作 — 用户
  function setUser(user: User) {
    currentUser.value = user
  }

  function clearUser() {
    currentUser.value = null
  }

  return {
    sidebarCollapsed,
    activeRoute,
    bottomNavVisible,
    currentUser,
    isLoggedIn,
    toggleSidebar,
    setActiveRoute,
    setBottomNavVisible,
    setUser,
    clearUser,
  }
})
