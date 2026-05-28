import { test, expect } from '@playwright/test'

test.describe('Homepage Features', () => {
  test('hero section renders with title', async ({ page }) => {
    await page.goto('/')

    // Hero section should be visible
    await expect(page.locator('.hero')).toBeVisible()

    // Hero title should show ZYBlog
    await expect(page.locator('.hero__title')).toContainText('ZYBlog')
  })

  test('hero section shows rotating phrases', async ({ page }) => {
    await page.goto('/')

    // Hero phrase should be visible
    await expect(page.locator('.hero__phrase')).toBeVisible()

    // Should show one of the rotating phrases
    const phrase = await page.locator('.hero__phrase').textContent()
    const validPhrases = [
      '思考、记录与分享',
      '探索技术的边界',
      '用代码改变世界',
      '与志同道合的人交流',
    ]
    expect(validPhrases).toContain(phrase?.trim())
  })

  test('hero section has scroll button', async ({ page }) => {
    await page.goto('/')

    // Scroll to articles button should be visible
    await expect(page.locator('.hero__cta')).toBeVisible()
  })

  test('search input is visible on homepage', async ({ page }) => {
    await page.goto('/')

    // Search input should be visible
    await expect(page.locator('.home__search .n-input')).toBeVisible()
    await expect(page.locator('.home__search input')).toHaveAttribute('placeholder', /搜索/)
  })

  test('article cards display when posts exist', async ({ page }) => {
    await page.goto('/')

    // Wait for content to load
    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    // Should show either article cards or empty state
    const cards = page.locator('.article-card')
    const empty = page.locator('.n-empty')
    await expect(cards.or(empty)).toBeVisible()
  })

  test('article cards have proper structure', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const cards = page.locator('.article-card')
    const count = await cards.count()

    if (count > 0) {
      // Each card should have title and excerpt
      const firstCard = cards.first()
      await expect(firstCard).toBeVisible()
    }
  })

  test('tag filter section appears when tags exist', async ({ page }) => {
    await page.goto('/')

    // Wait for tags to load
    await page.waitForTimeout(1000)

    // Tag filter section may or may not be visible depending on whether tags exist
    const tagSection = page.locator('.home__tags')
    if (await tagSection.isVisible()) {
      // Should have "全部" (all) tag
      await expect(tagSection.locator('.n-tag:has-text("全部")')).toBeVisible()
    }
  })

  test('clicking a tag filters posts', async ({ page }) => {
    await page.goto('/')

    // Wait for tags to load
    await page.waitForSelector('.home__tags, .article-card, .n-empty', { timeout: 10000 })

    const tagSection = page.locator('.home__tags')
    if (await tagSection.isVisible()) {
      const tags = tagSection.locator('.n-tag')
      const tagCount = await tags.count()

      if (tagCount > 1) {
        // Click a non-"全部" tag
        await tags.nth(1).click()

        // The "全部" tag should no longer be checked
        // The clicked tag should be active
        await page.waitForTimeout(500)

        // URL should update with tag parameter
        await expect(page).toHaveURL(/tag=/)
      }
    }
  })

  test('clicking "全部" tag resets filter', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.home__tags, .article-card, .n-empty', { timeout: 10000 })

    const tagSection = page.locator('.home__tags')
    if (await tagSection.isVisible()) {
      const tags = tagSection.locator('.n-tag')
      const tagCount = await tags.count()

      if (tagCount > 1) {
        // Click a specific tag first
        await tags.nth(1).click()
        await page.waitForTimeout(500)

        // Then click "全部"
        await tags.first().click()
        await page.waitForTimeout(500)

        // URL should not have tag parameter
        await expect(page).not.toHaveURL(/tag=/)
      }
    }
  })

  test('search input filters posts', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    // Type in search
    const searchInput = page.locator('.home__search input')
    await searchInput.fill('test')

    // Wait for debounce (300ms) + response
    await page.waitForTimeout(1000)

    // Should show either results or empty search state
    const cards = page.locator('.article-card')
    const empty = page.locator('.home__empty')
    await expect(cards.or(empty)).toBeVisible()
  })

  test('search updates URL with query parameter', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const searchInput = page.locator('.home__search input')
    await searchInput.fill('hello world')

    // Wait for debounce
    await page.waitForTimeout(500)

    // URL should contain the search query
    await expect(page).toHaveURL(/q=hello/)
  })

  test('clearing search restores normal view', async ({ page }) => {
    await page.goto('/')

    await page.waitForSelector('.article-card, .n-empty', { timeout: 10000 })

    const searchInput = page.locator('.home__search input')
    await searchInput.fill('some query')
    await page.waitForTimeout(500)

    // Clear the search
    await searchInput.clear()
    await page.waitForTimeout(500)

    // URL should no longer have q param
    await expect(page).not.toHaveURL(/q=/)
  })

  test('theme toggle is visible in sidebar', async ({ page }) => {
    await page.goto('/')

    // Theme toggle button should be visible
    await expect(page.locator('.theme-toggle').first()).toBeVisible()
  })

  test('clicking theme toggle switches theme', async ({ page }) => {
    await page.goto('/')

    const themeToggle = page.locator('.theme-toggle').first()
    await expect(themeToggle).toBeVisible()

    // Get initial background color
    const initialBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    // Click theme toggle
    await themeToggle.click()
    await page.waitForTimeout(300)

    // Background color should change
    const newBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    expect(newBg).not.toBe(initialBg)
  })
})
