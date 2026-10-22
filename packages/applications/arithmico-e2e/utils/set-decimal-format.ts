import { Page } from "@playwright/test";
import { navigate } from "./navigate";

export async function setDecimalFormat(page: Page, format: string) {
    await navigate(page, "/settings");
    const setting = page.getByTestId("decimal-format-setting");
    await setting.locator("button").click();
    await setting.getByText(format).click();
}
