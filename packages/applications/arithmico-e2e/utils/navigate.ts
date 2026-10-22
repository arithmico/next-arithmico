import { Page } from "@playwright/test";

export async function navigate(page: Page, path: string) {
    await page.locator(`.navigation-item a[href="${path}"]`)
        .click();
}