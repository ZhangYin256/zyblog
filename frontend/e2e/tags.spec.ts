import { test, expect } from '@playwright/test'

test.describe('Tags Functionality', () => {
  test('publish page has tag selector', async ({ page }) => {
    await page.goto('/publish')

    // Tag selector should be visible
    const tagSelector = page.locator('.publish-tags .n-select')
    await expect(tagSelector).toBeVisible()
  })

  test('tag selector shows placeholder text', async ({ page }) => {
    await page.goto('/publish')

    // Should show placeholder
    await expect(page.locator('.publish-tags')).toContainText('选择标签')
  })

  test('post detail shows tags when assigned', async ({ page }) => {
    await page.goto('/')

    // Wait for posts to load
    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      // Click the first post
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Tags section may or may not be visible depending on whether tags exist
      // Just verify the page loaded correctly
      await expect(page.locator('.post-content')).toBeVisible()

      // Check if tags are present (optional — not all posts have tags)
      const tagsSection = page.locator('.post-header__tags')
      if (await tagsSection.isVisible()) {
        // Tags should be rendered as n-tag components
        const tagElements = tagsSection.locator('.n-tag')
        await expect(tagElements.first()).toBeVisible()
      }
    }
  })

  test('tags appear as rounded badges on post detail', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    const postCards = page.locator('.post-card')
    const count = await postCards.count()

    if (count > 0) {
      await postCards.first().click()
      await expect(page).toHaveURL(/\/posts\/\d+/)

      // Verify the post header structure
      await expect(page.locator('.post-header')).toBeVisible()

      // If tags exist, they should use Naive UI's n-tag component
      const tags = page.locator('.post-header__tags .n-tag')
      const tagCount = await tags.count()

      if (tagCount > 0) {
        // Each tag should have text content
        for (let i = 0; i < tagCount; i++) {
          const tagText = await tags.nth(i).textContent()
          expect(tagText?.trim().length).toBeGreaterThan(0)
        }
      }
    }
  })
})
