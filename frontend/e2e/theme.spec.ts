import { test, expect } from '@playwright/test'

test.describe('Theme Toggle', () => {
  test('theme toggle button is visible in sidebar', async ({ page }) => {
    await page.goto('/')

    // Theme toggle button should be visible
    const themeToggle = page.locator('.theme-toggle')
    await expect(themeToggle.first()).toBeVisible()
  })

  test('clicking theme toggle switches between light and dark', async ({ page }) => {
    await page.goto('/')

    // Get the initial theme state
    const themeToggle = page.locator('.theme-toggle').first()
    await expect(themeToggle).toBeVisible()

    // Get initial body background color
    const initialBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    // Click theme toggle
    await themeToggle.click()

    // Wait for theme transition
    await page.waitForTimeout(300)

    // Background color should change
    const newBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    // The colors should be different (light vs dark)
    expect(newBg).not.toBe(initialBg)
  })

  test('theme preference persists in localStorage', async ({ page }) => {
    await page.goto('/')

    const themeToggle = page.locator('.theme-toggle').first()
    await expect(themeToggle).toBeVisible()

    // Get initial theme from localStorage
    const initialTheme = await page.evaluate(() => {
      return localStorage.getItem('zyblog_theme')
    })

    // Click toggle to switch theme
    await themeToggle.click()
    await page.waitForTimeout(300)

    // localStorage should have the new theme
    const newTheme = await page.evaluate(() => {
      return localStorage.getItem('zyblog_theme')
    })

    // Theme should have changed
    expect(newTheme).not.toBe(initialTheme)
  })

  test('theme persists after page reload', async ({ page }) => {
    await page.goto('/')

    const themeToggle = page.locator('.theme-toggle').first()
    await expect(themeToggle).toBeVisible()

    // Get initial background
    const initialBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    // Toggle theme
    await themeToggle.click()
    await page.waitForTimeout(300)

    // Get background after toggle
    const toggledBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    // Reload the page
    await page.reload()
    await page.waitForTimeout(500)

    // Background after reload should match the toggled state
    const reloadedBg = await page.evaluate(() => {
      return getComputedStyle(document.body).backgroundColor
    })

    expect(reloadedBg).toBe(toggledBg)
    expect(reloadedBg).not.toBe(initialBg)
  })

  test('theme toggle button text changes with theme', async ({ page }) => {
    await page.goto('/')

    const themeToggle = page.locator('.theme-toggle').first()
    await expect(themeToggle).toBeVisible()

    // Get initial button text
    const initialText = await themeToggle.textContent()

    // Click toggle
    await themeToggle.click()
    await page.waitForTimeout(300)

    // Button text should change (sun/moon icons)
    const newText = await themeToggle.textContent()
    expect(newText).not.toBe(initialText)
  })
})
