use crate::logger::logger as Logger;
pub mod logger;

fn main() {
    let console = Logger::LoggingService {
        log_level: String::from("dev"),
        name: String::from("main"),
        log_for: vec!["dev".to_string(), "stage".to_string()]
    };
    console.log([1, 2, 3]);
    console.error(String::from("Error Message"));
    console.warn(String::from("Warning Message"));
}
