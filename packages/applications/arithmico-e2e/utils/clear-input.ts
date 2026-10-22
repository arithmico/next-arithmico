import { Page } from "@playwright/test";
import { navigate } from "./navigate";

export async function clearInput(page: Page) {
    await navigate(page, "/");
    await page.getByTestId("calculator-actions")
        .locator("button")
        .click();
    await page.getByTestId("calculator-actions")
        .getByText("Clear Input")
        .click();
}
