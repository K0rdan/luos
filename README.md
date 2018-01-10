<img src="doc/img/logo-luos.png" width="300px">


# Luos: robotic product development made easy
[![Build Status](https://travis-ci.com/pollen/luos.svg?token=RxFY2cvxnBdyPk3Jevf4&branch=master)](https://travis-ci.com/pollen/luos)

Luos is an end-to-end system to fully orchestrate your robot, all the way from hardware control to high level behaviors and Apps management.

It includes out of the box sensors acquisitions and filtering drivers, advanced motor control for precise and eye-pleasing smooth motion, most commonly used robotic and AI algorithm and so on.

Its sandboxed multi-layer architectures and elegant APIs allow for efficient and productive programming.

**[more information on our website>>](https://www.luos.io/)**

## Architecture

Luos is built around 2 main concepts:
* **drivers** that defined standardized API as Rust traits for common robotics parts (e.g. a servo motor, a position encoder, a distance sensor, etc.). This enforce compatibility amongst drivers and let you seamlessly switch from one actuator/sensor implementation to another one without breaking all the rest of your code. 
* **core** represents a physical boards and its associated local drivers. It handles automatically the communication with the other cores - and thus the remote drivers - so you can develop your project with modularity.

## Example

A typical example of an app developed with Luos will look like this. You can define your own *drivers* based on the pre-defined traits or use already developed ones. Then, they can be easily composed to make complex behaviors.

```rust
extern crate luos;
use luos::{AngleEncoder, Motor};

extern crate luos_driver;
// We use a pre-defined driver for a known motor
use luos_driver::PololuluServoMotor;

// We define our implementation for our own encoder
struct MySpecificEncoder { ... }
impl AngleEncoder for MySpecificEncoder {
    // The API is standardized by traits
    // also enforcing units
    fn get_angle(&self) -> Degrees { ... }
}

fn main() {
    let encoder = MySpecificEncoder { pin: hal::GPIO::PA9 };
    let motor = PololuluServoMotor { pin: hal::GPIO::PB10 };

    asserv::loop(&encoder, &mut motor).run();
}
```

## Development

Luos is lightweight, it can run on low-cost microcontroller (from ARM Cortex M0).

It is currently developped on STM32F0 µC, you can use the following development board to test and use Luos.

- [Luos L0 (available soon)](https://www.luos.io/)
- [NUCLEO-F072RB](http://www.st.com/en/evaluation-tools/nucleo-f072rb.html)
- [32F072BDISCOVERY](http://www.st.com/en/evaluation-tools/32f072bdiscovery.html)


## License

This project is licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE)).

Please respect the attribution condition by adding a link to www.luos.io and the following *powered by luos* logo on any communication and documentation of your project.

<a href="doc/img/powered-by-luos.png"><img src="doc/img/powered-by-luos.png" width="250px"></a>

[This logo](doc/img/powered-by-luos.png) is also available in [black](doc/img/powered-by-luos-black.png) and [white](doc/img/powered-by-luos-white.png) for optimal integration in your design.
