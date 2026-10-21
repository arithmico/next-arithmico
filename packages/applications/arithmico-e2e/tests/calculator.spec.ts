import { test, expect } from "@playwright/test";
import { AxeBuilder } from "@axe-core/playwright";
import { setTheme } from "../utils/set-theme";
import { press, pressSequentially } from "../utils/press";

test("accessiblity checks (light theme)", async ({ page }) => {
  await setTheme(page, "Light");
  await page.goto("/");
  await page.locator(".page-title").waitFor();
  await page.locator(".theme-light").waitFor({
    state: "hidden"
  });
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});

test("accessiblity checks (dark theme)", async ({ page }) => {
  await setTheme(page, "Dark");
  await page.goto("/");
  await page.locator(".page-title").waitFor();
  await page.locator(".theme-dark").waitFor({
    state: "hidden"
  });
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});

test("has title", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/Arithmico/);
});

test("calculate 1 + 3", async ({ page }) => {
  await page.goto("/");
  const inputField = page.getByTestId("calculator-input")
  await pressSequentially(inputField, "1 + 2");
  await inputField.press("Enter");
  await expect(page.getByTestId("calculator-output")).toHaveText("3");
});

test("clear input", async ({ page }) => {
  await page.goto("/");
  const inputField = page.getByTestId("calculator-input");
  await pressSequentially(inputField, "1 + 2");
  await expect(inputField).toHaveText("1 + 2");

  const menu = page.getByTestId("calculator-actions");
  const button = menu.locator("button");

  await button.click();

  let action = menu.getByText("Clear Input");
  await action.click();

  await expect(inputField).toBeEmpty();
});

test("clear output", async ({ page }) => {
  await page.goto("/");
  const inputField = page.getByTestId("calculator-input");
  await pressSequentially(inputField, "1 + 2");
  await expect(inputField).toHaveText("1 + 2");
  await inputField.press("Enter");

  const output = page.getByTestId("calculator-output");
  await expect(output).toHaveText("3");

  const menu = page.getByTestId("calculator-actions");
  const button = menu.locator("button");

  await button.click();

  let action = menu.getByText("Clear Output");
  await action.click();

  await expect(output).toBeEmpty();
});
