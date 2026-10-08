# Migration of `fraction` and `mfraction` to Centralized NumberRepresentation Options

In previous versions of Arithmico v2, the `fraction` and `mfraction` functions were used separately to convert numbers into fractions or mixed fractions.

This approach had a few downsides:

* **Duplicated Work:** Similar logic was repeated across different functions.
* **Complex Usage:** Number formatting should ideally be a consistent behavior controlled globally via settings, rather than requiring separate function calls for each format.

## Decision

In Arithmico v3, we have removed the standalone functions `fraction` and `mfraction` and fully integrated their behavior into the number representation settings.

You can now choose your preferred number representation directly through the global settings:

* **Number** (with automatic scientific notation for very large or small numbers)
* **Fractions** (proper fractions)
* **Mixed Fractions** (with whole numbers and remainders)

## Consequences

### Positive

* **Industry Standard Alignment:** This brings Arithmico in line with other major scientific calculators, where formatting modes are global display settings rather than explicit conversion functions.
* **Accessibility Benefits for Braille Display Users:** By removing wrapper functions like `fraction(...)`, inputs become much shorter. This saves precious characters, resulting in cleaner output and fewer navigation steps on Braille displays.
* **Future Shortcut Integration:** We are planning to allow users to toggle these formats quickly via keyboard shortcuts, making adjustments much faster during calculations.
* **Consistent Behavior:** All display modes now share a uniform and reliable approach to handling edge cases.

### Negative

* **Breaking Change:** For users who previously typed `fraction(...)` or `mfraction(...)` directly in Arithmico v2, these functions no longer exist. You now simply switch the global display setting to your desired format.