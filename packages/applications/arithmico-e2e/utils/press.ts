import { expect, Locator } from "@playwright/test";

export async function press(locator: Locator, key: string) {
    const currentRevisionData = await locator.getAttribute("data-revision");
    if (!currentRevisionData) {
        throw new Error("Missing data-revision attribute!");
    }
    const currentRevision = parseInt(currentRevisionData, 10);

    await locator.press(key);

    await expect(async () => {
        const currentData = await locator.getAttribute("data-revision");
        const current = parseInt(currentData || "0", 10);
        expect(current).toBeGreaterThan(currentRevision);
    }).toPass({
        timeout: 5000,
        intervals: [10, 20, 50, 100]
    });
}

export async function pressSequentially(locator: Locator, text: string) {
    for (const char of text) {
        await press(locator, char);
    }
}