import { Page } from "@playwright/test";

export async function setNumberRepresentation(page: Page, value: string): Promise<void> {
    await page.goto("/settings");
    const setting = page.getByTestId("number-representation-setting");
    const button = setting.locator("button");
    await button.click();
    const options = setting.locator("ul");
    await options.getByText(value, { exact: true }).click();
}