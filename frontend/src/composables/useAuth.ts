import { ref, computed } from 'vue'

const STORAGE_KEY = 'zyblog_admin_key'

const adminKey = ref<string | null>(localStorage.getItem(STORAGE_KEY))

/**
 * Composable for admin authentication.
 * Stores the admin key in localStorage and provides reactive state.
 */
export function useAuth() {
  const isAuthenticated = computed(() => !!adminKey.value)

  function setAdminKey(key: string) {
    adminKey.value = key
    localStorage.setItem(STORAGE_KEY, key)
  }

  function clearAdminKey() {
    adminKey.value = null
    localStorage.removeItem(STORAGE_KEY)
  }

  function getAdminKey(): string | null {
    return adminKey.value
  }

  return {
    adminKey,
    isAuthenticated,
    setAdminKey,
    clearAdminKey,
    getAdminKey,
  }
}
