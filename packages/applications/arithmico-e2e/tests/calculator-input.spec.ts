import { test, expect } from "@playwright/test";
import { press, pressSequentially, withRevisionIncrease } from "../utils/press";

test("autofocus", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await expect(inputField).toBeFocused();
});

test("append text", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, "1 + 2");
    await expect(page.getByTestId("calculator-input")).toContainText("1 + 2");
});

test("insert text in between", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, "12");
    await press(inputField, "ArrowLeft");
    await pressSequentially(inputField, " + ");
    await expect(inputField).toContainText("1 + 2");
});

test("insert text at start", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, " + 2");
    for (let i = 0; i < 4; i++) {
        await press(inputField, "ArrowLeft");
    }
    await pressSequentially(inputField, "1");
    await expect(page.getByTestId("calculator-input")).toContainText("1 + 2");
});

test("highlight error trace", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, "1 + sin()");
    await press(inputField, "Enter");
    await expect(inputField).toHaveText("1 + sin()");
    let spans = inputField.locator("span");
    await expect(spans).toHaveCount(1);
    await expect(spans).toHaveText("sin()");
});

test("multi-byte UTF-16 character insertion", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, "1 + ");
    await withRevisionIncrease(inputField, async (inputField) => {
        await page.keyboard.type("😂");
    });
    await pressSequentially(inputField, " + 3");
    await expect(inputField).toHaveText("1 + 😂 + 3");
});

test("multi-byte UTF-16 character deletion", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await pressSequentially(inputField, "1 + ");
    await withRevisionIncrease(inputField, async (inputField) => {
        await page.keyboard.insertText("😂");
    });
    await expect(inputField).toHaveText("1 + 😂");
    await press(inputField, "Backspace");
    await expect(inputField).toHaveText("1 + ");
});
