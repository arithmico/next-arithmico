import { test, expect } from "@playwright/test";

test("default light theme", async ({ page }) => {
    await page.goto("/");
    await page.emulateMedia({ colorScheme: "no-preference" });

    const themeProvider = page.getByTestId("theme-provider");
    await expect(themeProvider).toHaveClass("theme-light");
});

test("detect light theme", async ({ page }) => {
    await page.goto("/");
    await page.emulateMedia({ colorScheme: "light" });

    const themeProvider = page.getByTestId("theme-provider");
    await expect(themeProvider).toHaveClass("theme-light");
});

test("detect dark theme", async ({ page }) => {
    await page.goto("/");
    await page.emulateMedia({ colorScheme: "dark" });

    const themeProvider = page.getByTestId("theme-provider");
    await expect(themeProvider).toHaveClass("theme-dark");
})