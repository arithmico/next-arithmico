import { test, expect } from "@playwright/test";

test("autofocus", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await expect(inputField).toBeFocused();
});

test("append text", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("1 + 2", {
        delay: 100
    });
    await expect(page.getByTestId("calculator-input")).toContainText("1 + 2");
});

test("insert text in between", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("12", {
        delay: 100
    });
    await inputField.press("ArrowLeft", {
        delay: 100
    });
    await inputField.pressSequentially(" + ", {
        delay: 100,
    });
    await expect(inputField).toContainText("1 + 2");
});

test("insert text at start", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially(" + 2", {
        delay: 100
    });
    for (let i = 0; i < 4; i++) {
        await inputField.press("ArrowLeft", {
            delay: 100
        });
    }
    await inputField.pressSequentially("1", {
        delay: 100
    });
    await expect(page.getByTestId("calculator-input")).toContainText("1 + 2");
});

test("highlight error trace", async ({ page }) => {
    await page.goto("/");
    const inputField = page.getByTestId("calculator-input");
    await inputField.pressSequentially("1 + sin()", {
        delay: 100
    });
    await inputField.press("Enter", {
        delay: 100
    });
    expect(await inputField.textContent()).toBe("1 + sin()");
    let spans = await inputField.locator("span").all();
    expect(spans.length == 1);
    expect(await spans[0].textContent()).toStrictEqual("sin()");
});