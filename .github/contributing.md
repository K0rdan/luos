# Contributing to Luos

👍 First off, thanks for taking the time to contribute! 👍

## What should I know before I get started?

As of now, the main source of information describing Luos key concepts and their articulation can be found on the [wiki](https://github.com/pollen/luos/wiki). We try to keep there the documentation up to date.

The three main Luos concepts are:

* the [Core](https://github.com/pollen/luos/wiki/Luos-Core) is the instance of Luos running on a board. It handles all the Modules and Apps interactions
* the set of [Modules](https://github.com/pollen/luos/wiki/Luos-Module) defining specific abstractions for the hardware access to sensors and actuators.
* the [Apps](https://github.com/pollen/luos/wiki/Luos-App) that let users write code on top of the Modules creating some behavior for the robot.

## How can I contribute?

### Reporting bugs

The main way to report a bug is by filling an [issue](https://github.com/pollen/luos/issues). First, please make sure the issue as not already be reported. The [fill out template](./issue_template.md) will guide you on what information will help us fix the bug.

### Did you intend to add a new feature?

If you intend to work on a new feature, the best way is to let us know as soon as possible by opening a Pull Request. Thus, we can discuss if this addition will fit well within Luos and guide you towards this.

## Submitting changes

### Styleguides

We use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy) to ensure Luos code respect the Rust styling guidelines.

We also follow the Rust naming conventions: https://aturon.github.io/style/naming.html
