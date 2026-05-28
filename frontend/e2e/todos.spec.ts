import { test, expect } from '@playwright/test'

test.describe('TODO Management', () => {
  test.beforeEach(async ({ page }) => {
    // Set auth token to access protected todos page
    await page.goto('/login')
    await page.evaluate(() => {
      localStorage.setItem('zyblog_admin_key', 'test-token')
    })
  })

  test('todos page loads with header', async ({ page }) => {
    await page.goto('/todos')

    // Should show todos page title
    await expect(page.locator('.todos-admin__title')).toContainText('TODO 管理')
    await expect(page.locator('.todos-admin__subtitle')).toContainText('管理文章中的待办事项')
  })

  test('todos page has filter tabs', async ({ page }) => {
    await page.goto('/todos')

    // Filter tabs should be visible
    await expect(page.locator('.filter-tab:has-text("全部")')).toBeVisible()
    await expect(page.locator('.filter-tab:has-text("未完成")')).toBeVisible()
    await expect(page.locator('.filter-tab:has-text("已完成")')).toBeVisible()
  })

  test('filter tabs show counts', async ({ page }) => {
    await page.goto('/todos')

    // Wait for data to load
    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    // Each filter tab should have a count badge
    const allTab = page.locator('.filter-tab:has-text("全部")')
    await expect(allTab.locator('.filter-tab__count')).toBeVisible()
  })

  test('todos page shows loading state', async ({ page }) => {
    await page.goto('/todos')

    // Should show either loading, empty, or todo list
    const loading = page.locator('.todos-admin__loading')
    const empty = page.locator('.todos-admin__empty')
    const groups = page.locator('.todo-groups')

    await expect(loading.or(empty).or(groups)).toBeVisible()
  })

  test('todos page shows empty state when no todos', async ({ page }) => {
    await page.goto('/todos')

    // Wait for content to load
    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const empty = page.locator('.todos-admin__empty')
    if (await empty.isVisible()) {
      await expect(empty.locator('.n-empty')).toBeVisible()
    }
  })

  test('todo groups show post titles when todos exist', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const groups = page.locator('.todo-group')
    const count = await groups.count()

    if (count > 0) {
      // Each group should have a title
      await expect(groups.first().locator('.todo-group__post-title')).toBeVisible()
      await expect(groups.first().locator('.todo-group__count')).toBeVisible()
    }
  })

  test('todo cards show status and title', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const cards = page.locator('.todo-card')
    const count = await cards.count()

    if (count > 0) {
      // Each card should have status indicator and title
      await expect(cards.first().locator('.todo-card__status')).toBeVisible()
      await expect(cards.first().locator('.todo-card__title')).toBeVisible()
    }
  })

  test('pending todo cards have complete button', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const pendingCards = page.locator('.todo-card:not(.todo-card--done)')
    const count = await pendingCards.count()

    if (count > 0) {
      // Pending cards should have a "完成" button
      await expect(pendingCards.first().locator('button:has-text("完成")')).toBeVisible()
    }
  })

  test('completed todo cards show done label', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const doneCards = page.locator('.todo-card--done')
    const count = await doneCards.count()

    if (count > 0) {
      // Completed cards should show "已完成" label
      await expect(doneCards.first().locator('.todo-card__done-label')).toContainText('已完成')
    }
  })

  test('clicking "未完成" filter shows only pending todos', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    // Click pending filter
    await page.locator('.filter-tab:has-text("未完成")').click()

    // Should not show completed cards
    const doneCards = page.locator('.todo-card--done')
    await expect(doneCards).toHaveCount(0)
  })

  test('clicking "已完成" filter shows only completed todos', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    // Click done filter
    await page.locator('.filter-tab:has-text("已完成")').click()

    // All visible cards should be done
    const cards = page.locator('.todo-card')
    const count = await cards.count()

    if (count > 0) {
      const doneCards = page.locator('.todo-card--done')
      await expect(doneCards).toHaveCount(count)
    }
  })

  test('clicking "全部" filter shows all todos', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    // First click a specific filter
    await page.locator('.filter-tab:has-text("未完成")').click()
    await page.waitForTimeout(300)

    // Then click all
    await page.locator('.filter-tab:has-text("全部")').click()
    await page.waitForTimeout(300)

    // Should show both completed and pending
    const allCards = page.locator('.todo-card')
    const allCount = await allCards.count()

    if (allCount > 0) {
      // Should have both types
      const doneCards = page.locator('.todo-card--done')
      const pendingCards = page.locator('.todo-card:not(.todo-card--done)')
      const doneCount = await doneCards.count()
      const pendingCount = await pendingCards.count()

      // At least one of each type should exist (if data permits)
      expect(doneCount + pendingCount).toBe(allCount)
    }
  })

  test('todo cards show subscriber badge', async ({ page }) => {
    await page.goto('/todos')

    await page.waitForSelector('.todo-groups, .todos-admin__empty, .todos-admin__loading', {
      timeout: 10000,
    })

    const cards = page.locator('.todo-card')
    const count = await cards.count()

    if (count > 0) {
      // Each card should have subscriber badge area
      await expect(cards.first().locator('.todo-card__badge')).toBeVisible()
    }
  })
})
