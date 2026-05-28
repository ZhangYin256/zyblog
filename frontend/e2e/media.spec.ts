import { test, expect } from '@playwright/test'

test.describe('Media Management', () => {
  test.beforeEach(async ({ page }) => {
    // Set auth token to access protected media page
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })
  })

  test('media page loads with header', async ({ page }) => {
    await page.goto('/media')

    // Should show media page title
    await expect(page.locator('.media-title')).toContainText('媒体管理')
    await expect(page.locator('.media-subtitle')).toBeVisible()
  })

  test('media page has upload button', async ({ page }) => {
    await page.goto('/media')

    // Upload button should be visible
    await expect(page.locator('button:has-text("上传文件")')).toBeVisible()
  })

  test('media page has drop zone', async ({ page }) => {
    await page.goto('/media')

    // Drop zone should be visible
    await expect(page.locator('.media-dropzone')).toBeVisible()
    await expect(page.locator('.media-dropzone__text')).toContainText('拖放文件到此处或点击上传')
  })

  test('drop zone shows supported file types', async ({ page }) => {
    await page.goto('/media')

    // Should show supported formats hint
    await expect(page.locator('.media-dropzone__hint')).toContainText('JPEG')
    await expect(page.locator('.media-dropzone__hint')).toContainText('PNG')
    await expect(page.locator('.media-dropzone__hint')).toContainText('MP4')
  })

  test('media page shows loading or content', async ({ page }) => {
    await page.goto('/media')

    // Should show either loading, empty state, or media grid
    const loading = page.locator('.media-loading')
    const empty = page.locator('.media-empty')
    const grid = page.locator('.media-grid')

    await expect(loading.or(empty).or(grid)).toBeVisible()
  })

  test('media page has hidden file input', async ({ page }) => {
    await page.goto('/media')

    // Hidden file input should exist (for upload functionality)
    const fileInput = page.locator('.media-hidden-input')
    await expect(fileInput).toHaveCount(1)

    // Should accept image and video types
    const accept = await fileInput.getAttribute('accept')
    expect(accept).toContain('image/')
    expect(accept).toContain('video/')
  })

  test('media grid shows file count when files exist', async ({ page }) => {
    await page.goto('/media')

    // Wait for content to load
    await page.waitForSelector('.media-grid, .media-empty, .media-loading', { timeout: 10000 })

    const grid = page.locator('.media-grid')
    if (await grid.isVisible()) {
      // Should show file count
      await expect(page.locator('.media-grid__count')).toBeVisible()
    }
  })

  test('media cards show file info when files exist', async ({ page }) => {
    await page.goto('/media')

    await page.waitForSelector('.media-grid, .media-empty, .media-loading', { timeout: 10000 })

    const cards = page.locator('.media-card')
    const count = await cards.count()

    if (count > 0) {
      // First card should have name and meta info
      await expect(cards.first().locator('.media-card__name')).toBeVisible()
      await expect(cards.first().locator('.media-card__meta')).toBeVisible()
    }
  })

  test('media cards have delete button', async ({ page }) => {
    await page.goto('/media')

    await page.waitForSelector('.media-grid, .media-empty, .media-loading', { timeout: 10000 })

    const cards = page.locator('.media-card')
    const count = await cards.count()

    if (count > 0) {
      // Delete button should be visible
      await expect(cards.first().locator('button:has-text("删除")')).toBeVisible()
    }
  })

  test('clicking delete shows confirmation popup', async ({ page }) => {
    await page.goto('/media')

    await page.waitForSelector('.media-grid, .media-empty, .media-loading', { timeout: 10000 })

    const cards = page.locator('.media-card')
    const count = await cards.count()

    if (count > 0) {
      // Click delete button
      await cards.first().locator('button:has-text("删除")').click()

      // Popconfirm should appear
      await expect(page.locator('.n-popconfirm')).toBeVisible()
    }
  })

  test('drop zone is clickable', async ({ page }) => {
    await page.goto('/media')

    // Drop zone should have pointer cursor (clickable)
    const dropzone = page.locator('.media-dropzone')
    await expect(dropzone).toBeVisible()

    // Clicking should trigger file input (we can verify it's interactive)
    const cursor = await dropzone.evaluate(el => getComputedStyle(el).cursor)
    expect(cursor).toBe('pointer')
  })
})
