import { test, expect } from '@playwright/test'

test.describe('Search Functionality', () => {
  test('search input is visible on home page', async ({ page }) => {
    await page.goto('/')

    // Search input should be visible
    const searchInput = page.locator('.home__search .n-input')
    await expect(searchInput).toBeVisible()
  })

  test('typing in search shows results or empty state', async ({ page }) => {
    await page.goto('/')

    // Wait for page to load
    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    // Type in search
    const searchInput = page.locator('.home__search input')
    await searchInput.fill('test query')

    // Wait for debounce (300ms) + API response
    await page.waitForTimeout(1000)

    // Should show either results or "no results" state
    const postList = page.locator('.post-list')
    const emptySearch = page.locator('.home__empty')

    await expect(postList.or(emptySearch)).toBeVisible()
  })

  test('search updates URL with query parameter', async ({ page }) => {
    await page.goto('/')

    // Wait for page to load
    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    // Type in search
    const searchInput = page.locator('.home__search input')
    await searchInput.fill('hello world')

    // Wait for debounce
    await page.waitForTimeout(500)

    // URL should contain the search query
    await expect(page).toHaveURL(/q=hello/)
  })

  test('clearing search restores normal post list', async ({ page }) => {
    await page.goto('/')

    // Wait for page to load
    await page.waitForSelector('.post-card, .n-empty', { timeout: 10000 })

    // Type in search
    const searchInput = page.locator('.home__search input')
    await searchInput.fill('some query')

    // Wait for debounce
    await page.waitForTimeout(500)

    // Clear the search
    await searchInput.clear()

    // Wait for debounce
    await page.waitForTimeout(500)

    // URL should no longer have q param
    await expect(page).not.toHaveURL(/q=/)
  })

  test('navigating to URL with q param restores search', async ({ page }) => {
    await page.goto('/?q=test')

    // Search input should have the value
    const searchInput = page.locator('.home__search input')
    await expect(searchInput).toHaveValue('test')
  })
})
