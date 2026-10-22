import { Page } from "@playwright/test";
import { navigate } from "./navigate";

export async function setAngleUnit(page: Page, unit: string) {
    await navigate(page, "/settings");
    const setting = page.getByTestId("angle-unit-setting");
    await setting.locator("button").click();
    await setting.locator("li").getByText(unit).click();
}
