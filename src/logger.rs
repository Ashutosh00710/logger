pub mod logger {
  use chrono::format::{DelayedFormat, StrftimeItems};

  /// ### Logger: Date and Time for Rust
  ///
  /// Example
  /// ```rust
  /// use crate::logger::logger as Logger;
  /// pub mod logger;
  ///
  /// let console = Logger::LoggingService {
  ///      log_level: String::from("dev"),
  ///      name: String::from("main")
  /// }
  ///
  /// console.log([1, 2, 3]);
  /// console.error(String::from("Error Message"));
  /// console.warn(String::from("Warning Message"));
  /// ```
  pub struct LoggingService {
    pub log_level: String,
    pub name: String
  }

  impl LoggingService {
    fn format(&self) -> (u32, DelayedFormat<StrftimeItems>, &String) {
      let now = chrono::Local::now();
      (
        std::process::id(),
        now.format("%b %e %Y %T"),
        &self.name,
      )
    }

    pub fn log<T>(&self, message: T)
      where T: std::fmt::Debug  {
      if self.log_level == "dev" {
        let (process_id, time, name) = self.format();
        println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[32m{:?}\x1B[39m", message);
      }
    }

    pub fn error<T>(&self, message: T)
      where T: std::fmt::Debug  {
      let (process_id, time, name) = self.format();
      println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[31m{:?}\x1B[39m", message);
    }

    pub fn warn<T>(&self, message: T)
      where T: std::fmt::Debug {
      if self.log_level == "dev" {
        let (process_id, time, name) = self.format();
        println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[33m{:?}\x1B[39m", message);
      }
    }
  }
}