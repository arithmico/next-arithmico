import test, { expect, Page } from "@playwright/test";
import { evaluate } from "../utils/evaluate";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";

async function setupDefinitions(page: Page) {
    await page.goto("/");
    await evaluate(page, "a := 1");
    await evaluate(page, "b := 2");
    await page.getByText("Definitions").click();
}

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/");
    await setupDefinitions(page);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/");
    await setupDefinitions(page);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("title", async ({ page }) => {
    await page.goto("/");
    await page.getByText("Definitions").click();
    await expect(page).toHaveURL("/definitions");
    const header = page.locator(".page-header .breadcrumbs-item");
    await expect(header).toContainText(["Calculator", "Definitions"]);
});
