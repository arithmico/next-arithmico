import { test, expect } from "@playwright/test";
import { setTheme } from "../utils/set-theme";
import AxeBuilder from "@axe-core/playwright";
import { evaluate } from "../utils/evaluate";
import { navigate } from "../utils/navigate";

test("accessiblity checks (light theme)", async ({ page }) => {
    await setTheme(page, "Light");
    await page.goto("/settings");
    await page.locator(".theme-light").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
    await setTheme(page, "Dark");
    await page.goto("/settings");
    await page.locator(".theme-dark").waitFor({
        state: "hidden"
    });
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
});

test("title", async ({ page }) => {
    await page.goto("/settings");
    await expect(page.locator(".page-title")).toContainText("Settings");
});

test("set theme", async ({ page }) => {
    await page.goto("/settings");
    await expect(page.locator(".page-title")).toContainText("Settings");

    const themeProvider = page.getByTestId("theme-provider");
    await expect(themeProvider).toHaveClass("theme-light");

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
    await expect(themeProvider).toHaveClass("theme-dark");
});

test("set language", async ({ page }) => {
    await page.goto("/settings");
    const title = page.locator(".page-title");
    await expect(title).toHaveText("Settings");

    const html = page.locator("html");
    await expect(html).toHaveAttribute("lang", "en");

    const setting = page.getByTestId("language-setting");
    await setting.locator("button").click();
    await setting.getByText("German").click();

    await expect(html).toHaveAttribute("lang", "de");
    await expect(title).toHaveText("Einstellungen");

    await page.reload();
    await expect(html).toHaveAttribute("lang", "de");
    await expect(title).toHaveText("Einstellungen");
});

test("set decimal format", async ({ page }) => {
    await page.goto("/settings");
    const title = page.locator(".page-title");
    await expect(title).toHaveText("Settings");
    const setting = page.getByTestId("decimal-format-setting");
    const button = setting.locator("button");
    await expect(button).toHaveText("No");
    await button.click();
    await setting.getByText("German").click();
    await page.reload();
    await expect(button).toHaveText("German");
});

test("set angle unit", async ({ page }) => {
    await page.goto("/settings");
    const title = page.locator(".page-title");
    await expect(title).toHaveText("Settings");
    const setting = page.getByTestId("angle-unit-setting");
    const button = setting.locator("button");
    await expect(button).toHaveText("Degrees");
    await button.click();
    await setting
        .locator("li")
        .getByText("Radians")
        .click();
    await page.reload();
    await expect(button).toHaveText("Radians");
});
