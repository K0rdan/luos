# Contributing to Luos

👍 First off, thanks for taking the time to contribute! 👍

## What should I know before I get started?

Luos is coded in [Rust](https://www.rust-lang.org) and built around 2 main concepts:

* **drivers** defines standardized API as Rust traits for common robotics parts (e.g. a servo motor, a position encoder, a distance sensor, etc.). This enforces compatibility amongst drivers and let you seamlessly switch from one actuator/sensor implementation to another without breaking the rest of your code.
* **core** represents a physical boards and its associated local drivers. It handles automatically the communication with the other cores - and thus the remote drivers - so you can develop your project with a modular approach.

## How can I contribute?

### Reporting bugs

The main way to report a bug is by filling an [issue](https://github.com/pollen-robotics/luos/issues). First, please make sure the issue is not already reported. The [issue template guide](./issue_template.md) will help you to fill an issue with the information that will help us fix the bug.

### Did you intend to add a new feature?

If you intend to work on a new feature, please read the [call for contributions](./call_for_contributions.md). The best way is to let us know as soon as possible by opening a pull request. Thus, we can discuss if this addition will fit well within Luos and guide you towards integrating it.

## Submitting changes

### Styleguides

We use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy) to ensure Luos code respect the Rust styling guidelines.

We also follow the Rust naming conventions: https://aturon.github.io/style/naming.html
