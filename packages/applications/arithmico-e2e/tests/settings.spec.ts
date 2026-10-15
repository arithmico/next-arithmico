import { test, expect } from "@playwright/test";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/settings");
    await page.waitForSelector(".page-title");
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/settings");
    await page.waitForSelector(".page-title");
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("Settings Title", async ({ page }) => {
    await page.goto("/settings");
    await expect(page.locator(".page-title")).toContainText("Settings");
});

test("Set Theme", async ({ page }) => {
    await page.goto("/settings");
    await expect(page.locator(".page-title")).toContainText("Settings");

    const setting = page.getByTestId("theme-setting");
    await expect(setting).toBeVisible();

    const button = setting.locator("button");
    await expect(button).toContainText("System");

    await button.click();

    const options = setting.locator("ul");
    await expect(options).toBeVisible();

    await page.getByText("Dark")
        .click();

    await page.reload();

    await expect(button).toContainText("Dark");
});
