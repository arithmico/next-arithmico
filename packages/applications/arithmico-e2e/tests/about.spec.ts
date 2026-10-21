import { test, expect } from "@playwright/test";
import { AxeBuilder } from "@axe-core/playwright";
import { setTheme } from "../utils/set-theme";

test("accessiblity checks (light theme)", async ({ page }) => {
  await setTheme(page, "Light");
  await page.goto("/about");
  await page.locator(".theme-light").waitFor({
    state: "hidden"
  });
  await expect(page.locator(".page-title")).toHaveText("About");
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
  await setTheme(page, "Dark");
  await page.goto("/about");
  await expect(page.locator(".page-title")).toHaveText("About");
  await page.locator(".theme-dark").waitFor({
    state: "hidden"
  });
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});
