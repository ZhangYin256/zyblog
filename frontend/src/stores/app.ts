import { defineStore } from 'pinia'
import { ref } from 'vue'

/**
 * 全局应用状态存储
 * 管理侧边栏可见性、主题偏好和 UI 状态
 */
export const useAppStore = defineStore('app', () => {
  // 侧边栏状态（仅 PC）
  const sidebarCollapsed = ref(false)

  // 当前活动路由键，用于菜单高亮
  const activeRoute = ref('/')

  // 移动端底部导航可见性
  const bottomNavVisible = ref(true)

  // 操作
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  function setActiveRoute(route: string) {
    activeRoute.value = route
  }

  function setBottomNavVisible(visible: boolean) {
    bottomNavVisible.value = visible
  }

  return {
    sidebarCollapsed,
    activeRoute,
    bottomNavVisible,
    toggleSidebar,
    setActiveRoute,
    setBottomNavVisible,
  }
})
