pub mod logger {
  use chrono::format::{DelayedFormat, StrftimeItems};

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

    pub fn log<T: std::fmt::Debug>(&self, message: T) {
      if self.log_level == "dev" {
        let (process_id, time, name) = self.format();
        println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[32m{:?}\x1B[39m", message);
      }
    }

    pub fn error<T: std::fmt::Debug>(&self, message: T) {
      let (process_id, time, name) = self.format();
      println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[31m{:?}\x1B[39m", message);
    }

    pub fn warn<T: std::fmt::Debug>(&self, message: T) {
      if self.log_level == "dev" {
        let (process_id, time, name) = self.format();
        println!("[{process_id}] - {time} - \x1B[33m[{name}]\x1B[39m \x1B[33m{:?}\x1B[39m", message);
      }
    }
  }
}