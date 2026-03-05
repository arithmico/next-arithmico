import { test, expect } from "@playwright/test";

test("load german system langauge", async ({ browser }) => {
    const context = await browser.newContext({
        locale: "de"
    });

    const page = await context.newPage();

    await page.goto("/settings");

    const testContainer = page.getByTestId("language-setting-value").first();
    await expect(testContainer).toContainText("Systemeinstellung");
});

test("load english system langauge", async ({ browser }) => {
    const context = await browser.newContext({
        locale: "en"
    });

    const page = await context.newPage();

    await page.goto("/settings");

    const testContainer = page.getByTestId("language-setting-value").first();
    await expect(testContainer).toContainText("System");
});
