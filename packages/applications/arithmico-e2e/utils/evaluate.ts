import { Page } from "@playwright/test";
import { pressSequentially } from "./press";
import { clearInput } from "./clear-input";

export async function evaluate(page: Page, input: string) {
    await clearInput(page);
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, input);
    await inputField.press("Enter");
}
