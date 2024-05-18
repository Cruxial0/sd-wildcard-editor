use crate::logging::log_level::LogLevel;

static DEBUG_ARG: &str = "debug";

pub(super) struct CliArguments {
    debug: bool
}

impl CliArguments {
    pub fn match_args(&mut self, args: tauri::Result<tauri::api::cli::Matches>){
        let matches = match args {
            Ok(m) => m,
            Err(_) => tauri::api::cli::Matches::default(),
        };

        for (arg, data) in matches.args {
            if arg == DEBUG_ARG { 
                println!("debug: {}", data.occurrences > 0);
                self.debug = data.occurrences > 0
            }
        }
    }

    pub fn get_log_level(&self) -> LogLevel {
        match self.debug {
            true => LogLevel::TRACE,
            false => LogLevel::INFO,
        }
    }
}

impl Default for CliArguments {
    fn default() -> Self {
        Self { 
            debug: false
        }
    }
}