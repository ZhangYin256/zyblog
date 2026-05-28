import { test, expect } from '@playwright/test'

test.describe('Comments Functionality', () => {
  test('post detail page shows comment section', async ({ page }) => {
    await page.goto('/')

    // Wait for posts to load
    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Comments section should be visible
      await expect(page.locator('.comments-section')).toBeVisible()
      await expect(page.locator('.comments-section__title')).toContainText('评论')
    }
  })

  test('comment form has required fields', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Comment form should have name field
      await expect(page.locator('#comment-name')).toBeVisible()

      // Comment form should have email field (optional)
      await expect(page.locator('#comment-email')).toBeVisible()

      // Comment form should have content field
      await expect(page.locator('#comment-content')).toBeVisible()

      // Submit button should exist
      await expect(page.locator('button:has-text("提交评论")')).toBeVisible()
    }
  })

  test('comment form validates required name field', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Try to submit without filling name
      await page.locator('button:has-text("提交评论")').click()

      // Should show warning message
      await expect(page.locator('.n-message')).toBeVisible()
    }
  })

  test('comment form validates required content field', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Fill name but not content
      await page.locator('#comment-name').fill('Test User')

      // Try to submit
      await page.locator('button:has-text("提交评论")').click()

      // Should show warning message
      await expect(page.locator('.n-message')).toBeVisible()
    }
  })

  test('submitting a comment shows pending message', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Fill in comment form
      await page.locator('#comment-name').fill('E2E Test User')
      await page.locator('#comment-email').fill('test@example.com')
      await page.locator('#comment-content').fill('This is an automated test comment.')

      // Submit comment
      await page.locator('button:has-text("提交评论")').click()

      // Should show success alert about pending approval
      await expect(page.locator('.n-alert--success')).toBeVisible()
      await expect(page.locator('.comment-form__success')).toContainText('等待审核')
    }
  })

  test('comments admin page shows pending comments', async ({ page }) => {
    await page.goto('/comments')

    // Should show the comments admin header
    await expect(page.locator('.comments-admin__title')).toContainText('评论管理')

    // Should show either comments or empty state
    const commentList = page.locator('.comment-list')
    const emptyState = page.locator('.n-empty')

    await expect(commentList.or(emptyState)).toBeVisible()
  })
})
