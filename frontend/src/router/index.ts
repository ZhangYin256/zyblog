import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuth } from '../composables/useAuth'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/publish',
    name: 'Publish',
    component: () => import('../views/Publish.vue')
  },
  {
    path: '/publish/:id',
    name: 'EditPost',
    component: () => import('../views/Publish.vue')
  },
  {
    path: '/posts',
    name: 'Posts',
    component: () => import('../views/ArticleList.vue')
  },
  {
    path: '/posts/:id',
    name: 'PostDetail',
    component: () => import('../views/PostDetail.vue')
  },
  {
    path: '/import',
    name: 'Import',
    component: () => import('../views/Import.vue')
  },
  {
    path: '/drafts',
    name: 'Drafts',
    component: () => import('../views/Drafts.vue')
  },
  {
    path: '/backup',
    name: 'Backup',
    component: () => import('../views/Backup.vue')
  },
  {
    path: '/comments',
    name: 'Comments',
    component: () => import('../views/Comments.vue')
  },
  {
    path: '/media',
    name: 'Media',
    component: () => import('../views/Media.vue')
  },
  {
    path: '/todos',
    name: 'Todos',
    component: () => import('../views/Todos.vue')
  },
  {
    path: '/tags',
    name: 'Tags',
    component: () => import('../views/Tags.vue')
  },
  {
    path: '/trash',
    name: 'Trash',
    component: () => import('../views/Trash.vue')
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('../views/Profile.vue')
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue')
  },
  {
    path: '/github/callback',
    name: 'GitHubCallback',
    component: () => import('../views/GitHubCallback.vue')
  },
  {
    path: '/my/subscriptions',
    name: 'MySubscriptions',
    component: () => import('../views/MySubscriptions.vue')
  },
  {
    path: '/my/pulls',
    name: 'MyPulls',
    component: () => import('../views/MyPulls.vue')
  },
  {
    path: '/my/comments',
    name: 'MyComments',
    component: () => import('../views/MyComments.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// ── Role-based route permissions ────────────────────────────

const rolePermissions: Record<string, string[]> = {
  visitor: ['/', '/posts', '/posts/:id', '/login', '/github/callback'],
  contributor: ['/', '/posts', '/posts/:id', '/my', '/my/subscriptions', '/my/pulls', '/my/comments', '/profile'],
  admin: ['*'], // all routes
}

// ── Navigation guard: role-based access control ────────────

router.beforeEach(async (to, _from, next) => {
  const { user, isAuthenticated, fetchMe } = useAuth()

  // Try to hydrate user state if token exists but user not loaded
  if (!user.value && isAuthenticated.value) {
    try {
      await fetchMe()
    } catch {
      // fetchMe failed (e.g. old ADMIN_KEY without JWT) — allow access as admin
    }
  }

  const userRole = user.value?.role || (isAuthenticated.value ? 'admin' : 'visitor')
  const allowedRoutes = rolePermissions[userRole]

  // Admin can access all routes
  if (allowedRoutes.includes('*')) {
    return next()
  }

  // Check if target route is in allowed list
  const isAllowed = allowedRoutes.some((route) => {
    if (route.includes(':')) {
      // Dynamic route matching (e.g. /posts/:id)
      const pattern = route.replace(/:(\w+)/g, '[^/]+')
      return new RegExp(`^${pattern}$`).test(to.path)
    }
    return to.path === route || to.path.startsWith(route + '/')
  })

  if (!isAllowed) {
    if (!isAuthenticated.value) {
      return next('/login')
    }
    return next('/') // redirect to home for authenticated but unauthorized
  }

  next()
})

export default router
