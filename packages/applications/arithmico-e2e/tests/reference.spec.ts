import test, { expect } from "@playwright/test";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/reference");
    await page.locator(".theme-light").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/reference");
    await page.locator(".theme-dark").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("search", async ({ page }) => {
    await page.goto("/reference");
    const search = page.locator(".reference-search");
    await search.fill("sin(x)");
    const items = page.locator(".reference-item");
    await expect(items.first()).toContainText("sin(x)");
    await expect(items.first()).toContainText("Calculate the sine of x.");
});
