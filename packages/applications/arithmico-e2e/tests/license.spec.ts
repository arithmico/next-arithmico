import test, { expect } from "@playwright/test";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/about/license");
    await page.waitForSelector(".breadcrumbs");
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/about/license");
    await page.waitForSelector(".breadcrumbs");
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});
