import { Page } from "@playwright/test";

export async function clearInput(page: Page) {
    await page.locator(".navigation-item > a")
        .getByText("Calculator")
        .click();
    await page.getByTestId("calculator-actions")
        .locator("button")
        .click();
    await page.getByTestId("calculator-actions")
        .getByText("Clear Input")
        .click();
}
