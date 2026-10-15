import { Page } from "@playwright/test";

export async function setTheme(page: Page, theme: "System" | "Dark" | "Light"): Promise<void> {
    await page.goto("/settings");
    const setting = page.getByTestId("theme-setting");
    const button = setting.locator("button");
    await button.click();
    const options = setting.locator("ul");
    await page.getByText(theme)
        .click();
}