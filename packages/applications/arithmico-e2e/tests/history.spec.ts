import test, { expect, Page } from "@playwright/test";
import { evaluate } from "../utils/evaluate";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";

async function setupHistory(page: Page) {
    await page.goto("/");
    await evaluate(page, "1 + 2");
    await evaluate(page, "2 + 3");
    await page.getByText("History").click();
}

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/");
    await setupHistory(page);
    await page.locator(".theme-light").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/");
    await setupHistory(page);
    await page.locator(".theme-dark").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("title", async ({ page }) => {
    await page.goto("/");
    await page.getByText("History").click();
    await expect(page).toHaveURL("/history");
    const header = page.locator(".page-header .breadcrumbs-item");
    await expect(header).toContainText(["Calculator", "History"]);
});

test("history items", async ({ page }) => {
    await page.goto("/");
    await evaluate(page, "1 + 2");
    await evaluate(page, "2 + 3");
    await page.getByText("History").click();
    await expect(page.locator(".history-item dd")).toHaveText([
        "1 + 2", "3",
        "2 + 3", "5"
    ]);
});
