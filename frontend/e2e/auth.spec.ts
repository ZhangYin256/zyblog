import { test, expect } from '@playwright/test'

test.describe('Authentication Flow', () => {
  test('login page renders with email and password fields', async ({ page }) => {
    await page.goto('/login')

    // Title should show login
    await expect(page.locator('.login-page__title')).toContainText('登录')
    await expect(page.locator('.login-page__subtitle')).toContainText('欢迎回来')

    // Email field
    await expect(page.locator('#email')).toBeVisible()

    // Password field
    await expect(page.locator('#password')).toBeVisible()

    // Submit button
    await expect(page.locator('.login-page__submit')).toContainText('登录')
  })

  test('login page has GitHub OAuth button', async ({ page }) => {
    await page.goto('/login')

    // GitHub login button should be visible
    await expect(page.locator('.login-page__github')).toBeVisible()
    await expect(page.locator('.login-page__github')).toContainText('使用 GitHub 登录')
  })

  test('toggle between login and register modes', async ({ page }) => {
    await page.goto('/login')

    // Start in login mode
    await expect(page.locator('.login-page__title')).toContainText('登录')

    // Click toggle to switch to register
    await page.locator('.login-page__toggle button').click()

    // Should now show register mode
    await expect(page.locator('.login-page__title')).toContainText('注册')
    await expect(page.locator('.login-page__subtitle')).toContainText('创建一个新账户')

    // Name field should appear in register mode
    await expect(page.locator('#name')).toBeVisible()

    // Submit button should say register
    await expect(page.locator('.login-page__submit')).toContainText('注册')

    // Toggle back to login
    await page.locator('.login-page__toggle button').click()

    await expect(page.locator('.login-page__title')).toContainText('登录')
    await expect(page.locator('#name')).not.toBeVisible()
  })

  test('login form validates empty fields', async ({ page }) => {
    await page.goto('/login')

    // Try to submit empty form
    await page.locator('.login-page__submit').click()

    // Should show warning message (Naive UI message)
    await expect(page.locator('.n-message')).toBeVisible()
  })

  test('register form validates missing name', async ({ page }) => {
    await page.goto('/login')

    // Switch to register mode
    await page.locator('.login-page__toggle button').click()
    await expect(page.locator('.login-page__title')).toContainText('注册')

    // Fill email and password but not name
    await page.locator('#email').fill('test@example.com')
    await page.locator('#password').fill('password123')

    // Submit
    await page.locator('.login-page__submit').click()

    // Should show warning about missing name
    await expect(page.locator('.n-message')).toBeVisible()
  })

  test('protected route redirects to login', async ({ page }) => {
    // Clear any stored auth
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.removeItem('zyblog_admin_key')
      localStorage.removeItem('zyblog_refresh_token')
    })

    // Try to access protected route
    await page.goto('/publish')

    // Should redirect to /login
    await expect(page).toHaveURL(/\/login/)
  })

  test('protected route /drafts redirects to login', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.removeItem('zyblog_admin_key')
      localStorage.removeItem('zyblog_refresh_token')
    })

    await page.goto('/drafts')
    await expect(page).toHaveURL(/\/login/)
  })

  test('protected route /backup redirects to login', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.removeItem('zyblog_admin_key')
      localStorage.removeItem('zyblog_refresh_token')
    })

    await page.goto('/backup')
    await expect(page).toHaveURL(/\/login/)
  })

  test('token persistence in localStorage', async ({ page }) => {
    await page.goto('/login')

    // Set a mock token
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token-123')
    })

    // Verify token persists
    const token = await page.evaluate(() => localStorage.getItem('zyblog_admin_key'))
    expect(token).toBe('test-token-123')

    // Navigate away and back
    await page.goto('/')
    const tokenAfterNav = await page.evaluate(() => localStorage.getItem('zyblog_admin_key'))
    expect(tokenAfterNav).toBe('test-token-123')
  })

  test('divider shows "or" text between login methods', async ({ page }) => {
    await page.goto('/login')

    await expect(page.locator('.login-page__divider')).toContainText('或')
  })
})
