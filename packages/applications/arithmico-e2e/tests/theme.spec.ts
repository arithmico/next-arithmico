import { test, expect } from "@playwright/test";

test("switch theme preference", async ({page}) => {
    await page.goto("/");

    const testContainer = page.getByTestId("theme-div").first();

    await page.emulateMedia({ colorScheme: "light" });
    await expect(testContainer).toHaveClass("theme-light");

    await page.emulateMedia({ colorScheme: "dark" });
    await expect(testContainer).toHaveClass("theme-dark");
});