import { test, expect } from "@playwright/test";

test("autofocus", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await expect(inputField).toBeFocused();
});

test("append text", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("1 + 2");
    await expect(inputField).toContainText("1 + 2");
});

test("insert text in between", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("12");
    await inputField.press("ArrowLeft");
    await inputField.pressSequentially(" + ");
    await expect(inputField).toContainText("1 + 2");
});

test("insert text at start", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially(" + 2");
    for (let i = 0; i < 4; i++) {
        await inputField.press("ArrowLeft");
    }
    await inputField.pressSequentially("1");
    await expect(inputField).toContainText("1 + 2");
});

test("highlight error trace", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("1 + sin()");
    await inputField.press("Enter");
    expect(await inputField.textContent()).toBe("1 + sin()");
    let spans = await inputField.locator("span").all();
    expect(spans.length == 1);
    expect(await spans[0].textContent()).toStrictEqual("sin()");
});