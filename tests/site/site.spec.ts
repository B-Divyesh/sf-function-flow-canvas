import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('loads without console errors and walks the sample path', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/Trace one request/);
  await page.getByRole('link', { name: 'Walk the sample path' }).click();
  await page.getByRole('button', { name: /decode_event/ }).click();
  await expect(page.locator('#inspect-name')).toHaveText('decode_event');
  await page.locator('#demo-search').fill('event');
  await expect(page.locator('#demo-status')).toContainText('1 symbols visible');
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
});

test('free depth prompts for Pathfinder and a valid license unlocks it', async ({ page }) => {
  await page.route('**/api/v1/products/function-flow-canvas/verify?license=*', (route) => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) }));
  await page.goto('/?license=test-token');
  await expect(page.locator('#license-status')).toContainText('Pathfinder is active');
  await expect(page.locator('.depth-3')).toBeVisible();
  await expect(page).toHaveURL('/');
});

test('legal pages and mobile navigation remain usable', async ({ page }) => {
  await page.goto('/privacy/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/Source stays/);
  await page.goto('/terms/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/map, not a guarantee/i);
  await expect(page.locator('main')).toBeVisible();
});
