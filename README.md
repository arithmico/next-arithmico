# Arithmico Project Repository

Welcome to the Arithmico project repository! This is a monorepo containing all the software packages for the Arithmico project.

## Project Description

Arithmico is a comprehensive science software meticulously crafted to meet the educational needs of visually impaired and blind students. 
Rooted in the principles of accessibility and inclusivity, Arithmico provides a versatile platform for learning and mastering various scientific disciplines. 
From mathematics to physics, chemistry, and beyond, Arithmico empowers students with intuitive tools and resources tailored to their unique learning requirements. 
By leveraging cutting-edge technologies and adhering to rigorous accessibility standards, Arithmico fosters an inclusive learning environment where every student can thrive and excel in their scientific pursuits.

## Overview

This repository utilizes Cargo's workspace feature to manage multiple packages within a single project. Each package serves a specific purpose in the Arithmico project:

- **arithmico**: The main application package.
- **engine**: Package for parsing and evaluating mathematical expressions.
- **headless_components**: Package containing accessible components for the Yew frontend framework.
- **icons**: Package providing Yew components for various icons.

## Development Setup

### Workspace Configuration

To manage the packages within this repository, Cargo's workspace feature is used. This allows for seamless development across multiple packages.

### Formatting

Code formatting is maintained using `rustfmt`, ensuring consistent and readable code throughout the repository.

### Commit Convention

Commit messages in this repository follow the Conventional Commits schema, providing a clear and standardized format for version control history.

## Getting Started

To get started with development, ensure you have Rust and Cargo installed on your system. 
Then, clone this repository and navigate to the desired package directory to begin development.
