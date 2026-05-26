import { defineStore } from 'pinia'
import { ref } from 'vue'

/**
 * Global application state store.
 * Manages sidebar visibility, theme preferences, and UI state.
 */
export const useAppStore = defineStore('app', () => {
  // Sidebar state (PC only)
  const sidebarCollapsed = ref(false)

  // Current active route key for menu highlighting
  const activeRoute = ref('/')

  // Mobile bottom nav visibility
  const bottomNavVisible = ref(true)

  // Actions
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
