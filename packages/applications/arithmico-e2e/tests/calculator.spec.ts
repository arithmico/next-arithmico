import { test, expect } from "@playwright/test";
import { AxeBuilder } from "@axe-core/playwright";
import { setTheme } from "../utils/set-theme";
import { press, pressSequentially } from "../utils/press";
import { setDecimalFormat } from "../utils/set-decimal-format";
import { evaluate } from "../utils/evaluate";
import { setAngleUnit } from "../utils/set-angle-unit";
import { setNumberRepresentation } from "../utils/set-number-representation";
import { setDecimalPlaces } from "../utils/set-decimal-places";

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

  const action = menu.getByText("Clear Output");
  await action.click();

  await expect(output).toBeEmpty();
});

test("output with english decimal format", async ({ page }) => {
  await page.goto("/");
  await setDecimalFormat(page, "English");
  await evaluate(page, "1 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("0.33333");
});

test("output with german decimal format", async ({ page }) => {
  await page.goto("/");
  await setDecimalFormat(page, "German");
  await evaluate(page, "1 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("0,33333");
});

test("output with angle unit set to degrees", async ({ page }) => {
  await page.goto("/");
  await setAngleUnit(page, "Degrees");
  await evaluate(page, "sin(90)");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("1");
});

test("output with angle unit set to radians", async ({ page }) => {
  await page.goto("/");
  await setAngleUnit(page, "Radians");
  await evaluate(page, "sin(pi / 2)");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("1");
});

test("output with number representation set to number", async ({ page }) => {
  await page.goto("/");
  await setNumberRepresentation(page, "Number");
  await evaluate(page, "4 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("1.33333");
});

test("output with number representation set to fraction", async ({ page }) => {
  await page.goto("/");
  await setNumberRepresentation(page, "Fraction");
  await evaluate(page, "4 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("4 / 3");
});

test("output with number representation set to mixed fraction", async ({ page }) => {
  await page.goto("/");
  await setNumberRepresentation(page, "Mixed fraction");
  await evaluate(page, "4 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("1 + 1 / 3");
});

test("reset session", async ({ page }) => {
  await page.goto("/");
  const inputField = page.getByTestId("calculator-input");
  await pressSequentially(inputField, "a := 2");
  await inputField.press("Enter");

  const output = page.getByTestId("calculator-output");
  await expect(output).toHaveText("a := 2");

  const menu = page.getByTestId("calculator-actions");
  const button = menu.locator("button");

  await button.click();

  await menu.getByText("Reset Session").click();

  await expect(output).toBeEmpty();

  await pressSequentially(inputField, "a");
  await inputField.press("Enter");
  await expect(output).toContainText(`Unknown symbol: "a"`);
});

test("output with decimal places set to 5", async ({ page }) => {
  await page.goto("/");
  await setDecimalPlaces(page, "5");
  await evaluate(page, "1 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("0.33333");
});

test("output with decimal places set to 3", async ({ page }) => {
  await page.goto("/");
  await setDecimalPlaces(page, "3");
  await evaluate(page, "1 / 3");
  const output = page.locator(".calculator-output")
  await expect(output).toHaveText("0.333");
});
