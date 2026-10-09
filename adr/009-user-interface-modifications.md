# User Interface Modifications

In Arithmico v2, the user interface included a top navigation bar, a read-only text input field for displaying computation outputs, and a single flat reference index for listing functions and constants. As Arithmico expanded its computational capabilities—specifically adding support for 2D function plotting—these UI patterns introduced severe usability and structural limitations:

* The top navigation bar consumed vertical viewport height, leaving insufficient vertical space for 2D graphics. This forced graphs to render at reduced sizes with distorted aspect ratios, degrading readability.

* Displaying computation outputs within a read-only `<input>` element represented an anti-pattern that violated web accessibility standards.

* The single-index reference page required manual explanations of signature details (such as argument types and argument cardinality) directly inside description text.

## Decision

We decided to overhaul the UI structure in Arithmico v3 with the following changes:

1. **Side Navigation Bar:** Moved the main navigation bar from the top to the side to increase available vertical display space for 2D function plotting and graphical outputs.

2. **Accessible `<output>` Element:** Replaced the read-only text input field with a semantic HTML `<output>` element to align with web accessibility standards.

3. **Dedicated Reference Detail Pages:** Enhanced the reference pages by maintaining the entry index page while adding dedicated details pages for each function and constant. Every argument's type and cardinality in a function's signature are now automatically displayed on its details page.

## Alternatives Considered

* **Replicate Arithmico v2 UI Behavior:** We considered retaining the v2 top navigation, read-only text input for outputs, and single-index reference page. This was rejected because it maintained vertical space constraints for plotting, preserved accessibility violations, and required repetitive, manual signature documentation in reference descriptions.

## Consequences

## Positive

* Significantly increased vertical viewport space for rendering clear, properly proportioned 2D graphics.

* Improved screen reader compatibility and semantic HTML compliance by utilizing native `<output>` elements.

* Standardized, automated rendering of function signatures, argument types, and parameter cardinalities across reference pages.

## Negative

* Horizontal screen real estate is slightly reduced on narrower screens due to the side navigation panel.

* Routing and component state management complexity increased due to introducing nested reference detail routes.
