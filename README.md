# Arithmico Project Repository

Welcome to the Arithmico project repository! This is a monorepo containing all the software packages for the Arithmico project.

## Project Description

Arithmico is a comprehensive science software meticulously crafted to meet the educational needs of visually impaired and blind students. 
Rooted in the principles of accessibility and inclusivity, Arithmico provides a versatile platform for learning and mastering various scientific disciplines. 
From mathematics to physics, chemistry, and beyond, Arithmico empowers students with intuitive tools and resources tailored to their unique learning requirements. 
By leveraging cutting-edge technologies and adhering to rigorous accessibility standards, Arithmico fosters an inclusive learning environment where every student can thrive and excel in their scientific pursuits.

## Overview

This repository is organized as a Cargo workspace, allowing for the coordinated development of multiple Rust packages within the broader Arithmico project. Each package plays a distinct role, contributing to various aspects of the application's functionality—from core computation to frontend components.

Below is an overview of the packages contained in this workspace:

| Package Name         | Path                         | README                                                   |
|----------------------|------------------------------|----------------------------------------------------------|
| arithmico            | /applications/arithmico      | [README](./packages/applications/arithmico/README.md)      |
| cssbundler           | /applications/cssbundler     | [README](./packages/applications/cssbundler/README.md)     |
| create_version       | /applications/create_version | [README](./packages/applications/create_version/README.md) |
| editor               | /libraries/editor            | -                                                        |
| editor_core          | /libraries/editor_core       | -                                                        |
| engine               | /libraries/engine            | -                                                        |
| trace                | /libraries/trace             | -                                                        |
| translate            | /libraries/translate         | -                                                        |
| translate_core       | /libraries/translate_core    | -                                                        |
| ui                   | /libraries/ui                | -                                                        |
| web_state            | /libraries/web_state         | -                                                        |

## Development Setup

### Workspace Configuration

To manage the packages within this repository, Cargo's workspace feature is used. This allows for seamless development across multiple packages.

### Formatting

Code formatting is maintained using `rustfmt` and `leptosfmt`, ensuring consistent and readable code throughout the repository.

### Commit Convention

Commit messages in this repository follow the Conventional Commits schema, providing a clear and standardized format for version control history.

## Getting Started

To get started with development, ensure you have Rust and Cargo installed on your system. 
Then, clone this repository and navigate to the desired package directory to begin development.
