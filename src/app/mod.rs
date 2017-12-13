//! Luos App - used to expose higher-level behaviors or APIs
//!
//! App aimed at exposing higher-level behaviors or APIs on top of driver. For instance, by combining various drivers (e.g. `AngleSensor` on wheels) you can compute richer information such as the odometry of a 2D wheeled robot.
//!
//! Similarly to `Driver` which are sandboxed, an App can't access other Apps but only other Drivers.

mod odometry;

pub use self::odometry::PlanaryLocalizer;
