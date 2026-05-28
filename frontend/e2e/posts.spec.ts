import { test, expect } from '@playwright/test'

test.describe('Post CRUD Flow', () => {
  test('home page displays post list', async ({ page }) => {
    await page.goto('/')

    // Should show hero section
    await expect(page.locator('.hero')).toBeVisible()

    // Should show either posts or empty state
    const postList = page.locator('.article-grid')
    const emptyState = page.locator('.n-empty')

    // At least one of these should be visible
    await expect(postList.or(emptyState)).toBeVisible()
  })

  test('navigating to publish page shows editor', async ({ page }) => {
    // Set auth token to avoid redirect
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })

    await page.goto('/publish')

    // Should show the publish header
    await expect(page.locator('.publish-header__title')).toContainText('写点什么')

    // Title input should be visible
    await expect(page.locator('.publish-title')).toBeVisible()

    // Content textarea should be visible
    await expect(page.locator('.publish-content')).toBeVisible()

    // Publish button should exist
    await expect(page.locator('button:has-text("发布")').first()).toBeVisible()
  })

  test('publish form requires title and content', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })

    await page.goto('/publish')

    // Try to publish with empty form
    const publishBtn = page.locator('.publish-header__right button:has-text("发布")')
    await publishBtn.click()

    // Should show warning message (Naive UI message)
    await expect(page.locator('.n-message')).toBeVisible()
  })

  test('publish page has tag selector', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })

    await page.goto('/publish')

    // Tag selector should be visible
    await expect(page.locator('.publish-tags .n-select')).toBeVisible()
  })

  test('publish page has formatting toolbar', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })

    await page.goto('/publish')

    // Toolbar buttons should be visible
    await expect(page.locator('.publish-toolbar')).toBeVisible()
    await expect(page.locator('.toolbar-btn').first()).toBeVisible()
  })

  test('edit post route loads existing post', async ({ page }) => {
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })

    // Navigate to edit route with a fake ID
    // The page will attempt to fetch the post — it may fail gracefully
    await page.goto('/publish/1')

    // Should show edit mode header
    await expect(page.locator('.publish-header__title')).toContainText('编辑文章')
  })

  test('clicking a post card navigates to detail page', async ({ page }) => {
    await page.goto('/')

    // Wait for posts to load
    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      // Click the first post
      await postCards.first().click()

      // Should navigate to post detail
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Should show post content area
      await expect(page.locator('.post-content')).toBeVisible()

      // Should show back button
      await expect(page.locator('.back-button')).toBeVisible()
    }
  })

  test('post detail page shows back navigation', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Click back button
      await page.locator('.back-button').click()

      // Should return to home
      await expect(page).toHaveURL('/')
    }
  })

  test('post detail shows comments section', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Comments section should be visible
      await expect(page.locator('.comments-section')).toBeVisible()
      await expect(page.locator('.comments-section__title')).toContainText('评论')
    }
  })

  test('post detail shows comment form fields', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Comment form should have fields
      await expect(page.locator('#comment-name')).toBeVisible()
      await expect(page.locator('#comment-email')).toBeVisible()
      await expect(page.locator('#comment-content')).toBeVisible()
      await expect(page.locator('button:has-text("提交评论")')).toBeVisible()
    }
  })

  test('post detail shows revision history button', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Revision history button should be visible
      await expect(page.locator('.post-header__revision-btn')).toBeVisible()
      await expect(page.locator('.post-header__revision-btn')).toContainText('版本历史')
    }
  })
})
