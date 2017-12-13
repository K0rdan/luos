//! Monitor system with events and handlers.
//!
//! # Examples
//! ```
//! extern crate luos;
//!
//! use luos::monitor::EventsHandler;
//!
//! struct Logger {}
//! impl Logger {
//!     pub fn new() -> Logger {
//!         Logger {}
//!     }
//! }
//! impl EventsHandler for Logger {
//!     fn on_fatal(&self, s: &str) {
//!         println!("FATAL: {:?}", s);
//!     }
//!     fn on_error(&self, s: &str) {
//!         println!("ERROR: {:?}", s);
//!     }
//!     fn on_warn(&self, s: &str) {
//!         println!("WARN: {:?}", s);
//!     }
//!     fn on_info(&self, s: &str) {
//!         println!("INFO: {:?}", s);
//!     }
//!     fn on_debug(&self, s: &str) {
//!         println!("DEBUG: {:?}", s);
//!     }
//!     fn on_trace(&self, s: &str) {
//!         println!("TRACE: {:?}", s);
//!     }
//! }
//!
//! fn main() {
//!     luos::monitor::register_handler(Logger::new());
//!
//!     for i in 0..10 {
//!         luos::monitor::info(&format!("I'm in the {:?} iteration of the loop!", i));
//!     }
//! }
//! ```

use alloc::vec::Vec;
use alloc::boxed::Box;

/// Events Handler trait that will receive the different level notifications
///
/// # Examples
///
/// ```
/// use luos::monitor::EventsHandler;
///
/// struct Logger {}
/// impl Logger {
///     fn notify(&self, lvl: &str, msg: &str) {
///         println!("{:?} - {:?}", lvl, msg);
///     }
/// }
/// impl EventsHandler for Logger {
///     fn on_fatal(&self, msg: &str) {
///         self.notify("FATAL", msg);
///     }
///     fn on_error(&self, msg: &str) {
///         self.notify("ERROR", msg);
///     }
///     fn on_warn(&self, msg: &str) {
///         self.notify("WARN", msg);
///     }
///     fn on_info(&self, msg: &str) {
///         self.notify("INFO", msg);
///     }
///     fn on_debug(&self, msg: &str) {
///         self.notify("DEBUG", msg);
///     }
///     fn on_trace(&self, msg: &str) {
///         self.notify("TRACE", msg);
///     }
/// }
/// ```
pub trait EventsHandler {
    /// Receive a fatal event with message
    fn on_fatal(&self, msg: &str);
    /// Receive an error event with message
    fn on_error(&self, msg: &str);
    /// Receive a warn event with message
    fn on_warn(&self, msg: &str);
    /// Receive an info event with message
    fn on_info(&self, msg: &str);
    /// Receive a debug event with message
    fn on_debug(&self, msg: &str);
    /// Receive a trace event with message
    fn on_trace(&self, msg: &str);
}

static mut HANDLERS: Option<Vec<Box<EventsHandler + Sync + 'static>>> = None;

/// Register a new `EventsHandler`
///
/// As soon as it is registred the handler will start receive new event notifications.
pub fn register_handler<H: EventsHandler + Sync + 'static>(h: H) {
    let h = Box::new(h);

    unsafe {
        if let Some(ref mut handlers) = HANDLERS {
            handlers.push(h);
        } else {
            HANDLERS = Some(vec![h]);
        }
    }
}

/// Publish a fatal event with message
pub fn fatal(msg: &str) {
    notify(LogLevel::Fatal, msg)
}
/// Publish an error event with message
pub fn error(msg: &str) {
    notify(LogLevel::Error, msg)
}
/// Publish a warn event with message
pub fn warn(msg: &str) {
    notify(LogLevel::Warn, msg)
}
/// Publish an info event with message
pub fn info(msg: &str) {
    notify(LogLevel::Info, msg)
}
/// Publish a debug event with message
pub fn debug(msg: &str) {
    notify(LogLevel::Debug, msg)
}
/// Publish a trace event with message
pub fn trace(msg: &str) {
    notify(LogLevel::Trace, msg)
}

#[allow(unused)]
enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn notify(lvl: LogLevel, msg: &str) {
    unsafe {
        if let Some(ref handlers) = HANDLERS {
            for h in handlers {
                match lvl {
                    LogLevel::Fatal => h.on_fatal(msg),
                    LogLevel::Error => h.on_error(msg),
                    LogLevel::Warn => h.on_warn(msg),
                    LogLevel::Info => h.on_info(msg),
                    LogLevel::Debug => h.on_debug(msg),
                    LogLevel::Trace => h.on_trace(msg),
                }
            }
        }
    }
}
