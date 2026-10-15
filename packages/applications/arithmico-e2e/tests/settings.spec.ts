import { test, expect } from "@playwright/test";

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
