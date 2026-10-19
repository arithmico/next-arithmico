import { Page } from "@playwright/test";

export async function clearInput(page: Page) {
    await page.goto("/");
    const menu = page.getByTestId("calculator-actions");
    const button = menu.locator("button");
    await button.click();
    let action = menu.getByText("Clear Input");
    await action.click();
}
