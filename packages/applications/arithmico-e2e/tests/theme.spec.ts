import test, { expect } from "@playwright/test";

test("switch theme preference", async ({page}) => {
    await page.goto("/");

    const testContainer = page.getByTestId("calculator-input").first();

    await page.emulateMedia({ colorScheme: "dark" });
    await expect(testContainer).toHaveCSS(
        "background-color",
        "oklch(0.205 0 0)" // --color-neutral-900
    );

    await page.emulateMedia({ colorScheme: "light" });
    await expect(testContainer).toHaveCSS(
        "background-color",
        "oklch(0.97 0 0)" // --color-neutral-100
    );
});