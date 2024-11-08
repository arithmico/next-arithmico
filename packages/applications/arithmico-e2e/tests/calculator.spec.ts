import { test, expect } from "@playwright/test";

test("has title", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/Arithmico/);
});

test("calculate 1 + 3", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("calculator-input").pressSequentially("1 + 2");
  await page.getByTestId("calculator-input").press("Enter");
  await expect(page.getByTestId("calculator-output")).toHaveText("3");
});
