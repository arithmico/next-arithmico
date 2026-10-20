# Accessibility at Arithmico

As a software project designed specifically for visually impaired and blind students, accessibility is not just a feature of Arithmico - it is the foundation of our entire application. We are committed to ensuring our scientific calculator and educational tools are fully operable and perceivable for everyone.

## Automated Accessibility Testing

To prevent regressions and ensure a high baseline of accessibility across all updates, we enforce automated accessibility testing in our continuous integration pipeline. 

* **Playwright & axe-core:** We use Playwright in combination with `axe-core` to automatically audit all application routes and user workflows for accessibility violations during our end-to-end testing phase.

## Compliance and Auditing

We are committed to complying with the German accessibility standard **BITV 2.0** (Barrierefreie-Informationstechnik-Verordnung), which builds upon WCAG guidelines. 

While we heavily rely on automated tools for continuous monitoring, we recognize that true accessibility requires human evaluation. We are planning to undergo a comprehensive, formal accessibility audit performed by a certified auditor in the future to officially certify Arithmico.

## Reporting Accessibility Issues

If you encounter an accessibility barrier or have a suggestion to improve screen reader compatibility, keyboard navigation, or overall usability, please let us know.

Please [open an issue](https://github.com/arithmico/next-arithmico/issues/new) on our GitHub repository with the label `a11y`. We prioritize these reports to ensure Arithmico remains a reliable tool for all students.
