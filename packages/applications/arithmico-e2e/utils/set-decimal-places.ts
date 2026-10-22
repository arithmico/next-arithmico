import { Page } from "@playwright/test";

export async function setDecimalPlaces(page: Page, value: string): Promise<void> {
    await page.goto("/settings");
    const setting = page.getByTestId("decimal-places-setting");
    const button = setting.locator("button");
    await button.click();
    const options = setting.locator("ul");
    await options.getByText(value, { exact: true }).click();
}
