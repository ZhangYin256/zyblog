import { test, expect } from '@playwright/test'

test.describe('Version Control (Revision History)', () => {
  test('post detail page has version history button', async ({ page }) => {
    await page.goto('/')

    // Wait for posts to load
    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Version history button should be visible in the post header
      const revisionBtn = page.locator('.post-header__revision-btn')
      await expect(revisionBtn).toBeVisible()
      await expect(revisionBtn).toContainText('版本历史')
    }
  })

  test('clicking version history button opens revision panel', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Click the version history button
      await page.locator('.post-header__revision-btn').click()

      // Revision panel should appear
      await expect(page.locator('.revision-panel')).toBeVisible()
      await expect(page.locator('.revision-panel__title')).toContainText('版本历史')
    }
  })

  test('revision panel has close button', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      // Close button should be visible
      await expect(page.locator('.revision-panel__close')).toBeVisible()

      // Click close
      await page.locator('.revision-panel__close').click()

      // Panel should be hidden
      await expect(page.locator('.revision-panel')).not.toBeVisible()
    }
  })

  test('revision panel shows loading state', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      // Should show either loading spinner, empty state, or revision list
      const loading = page.locator('.revision-panel__loading')
      const empty = page.locator('.revision-panel__empty')
      const list = page.locator('.revision-list')

      await expect(loading.or(empty).or(list)).toBeVisible()
    }
  })

  test('revision panel shows revision items when available', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      // Wait for panel content to load
      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 0) {
        // Each revision should have a version label
        await expect(revisionItems.first().locator('.revision-item__version')).toBeVisible()

        // Each revision should have a timestamp
        await expect(revisionItems.first().locator('.revision-item__time')).toBeVisible()
      }
    }
  })

  test('revision items have rollback button', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 0) {
        // Rollback button should be visible on revision items
        await expect(revisionItems.first().locator('button:has-text("回滚")')).toBeVisible()
      }
    }
  })

  test('clicking rollback shows confirmation dialog', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 0) {
        // Click rollback on first revision
        await revisionItems.first().locator('button:has-text("回滚")').click()

        // Confirmation dialog should appear
        await expect(page.locator('.rollback-dialog')).toBeVisible()
        await expect(page.locator('.rollback-dialog__title')).toContainText('确认回滚')

        // Dialog should have cancel and confirm buttons
        await expect(page.locator('.rollback-dialog__actions button:has-text("取消")')).toBeVisible()
        await expect(page.locator('.rollback-dialog__actions button:has-text("确认回滚")')).toBeVisible()
      }
    }
  })

  test('rollback dialog can be cancelled', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 0) {
        await revisionItems.first().locator('button:has-text("回滚")').click()
        await expect(page.locator('.rollback-dialog')).toBeVisible()

        // Click cancel
        await page.locator('.rollback-dialog__actions button:has-text("取消")').click()

        // Dialog should be hidden
        await expect(page.locator('.rollback-dialog')).not.toBeVisible()
      }
    }
  })

  test('revision items show diff button for older revisions', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 1) {
        // Non-latest revisions should have a "查看 Diff" button
        const diffButton = revisionItems.nth(1).locator('button:has-text("查看 Diff")')
        await expect(diffButton).toBeVisible()
      }
    }
  })

  test('clicking diff opens diff view modal', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 1) {
        // Click diff button
        await revisionItems.nth(1).locator('button:has-text("查看 Diff")').click()

        // Diff modal should appear
        await expect(page.locator('.diff-modal')).toBeVisible()
        await expect(page.locator('.diff-modal__title')).toContainText('版本对比')
      }
    }
  })

  test('diff modal has close button', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.article-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await page.locator('.post-header__revision-btn').click()

      await page.waitForSelector('.revision-list, .revision-panel__empty', { timeout: 10000 })

      const revisionItems = page.locator('.revision-item')
      const revCount = await revisionItems.count()

      if (revCount > 1) {
        await revisionItems.nth(1).locator('button:has-text("查看 Diff")').click()
        await expect(page.locator('.diff-modal')).toBeVisible()

        // Close button should work
        await page.locator('.diff-modal__close').click()
        await expect(page.locator('.diff-modal')).not.toBeVisible()
      }
    }
  })
})
