use std::ffi::OsStr;
use std::process::ExitCode;

use myhostid::{config_path, parse_args, read_host_id, Action, ArgumentError};

const HELP: &str = concat!(
    "Usage: hostid [OPTION]\n",
    "Print the configured identifier for the current host.\n",
    "\n",
    "      --help        display this help and exit\n",
    "      --version     output version information and exit\n",
);

fn main() -> ExitCode {
    let args = std::env::args_os().skip(1);

    match parse_args(args) {
        Ok(Action::PrintHostId) => print_host_id(),
        Ok(Action::Help) => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Ok(Action::Version) => {
            println!("hostid {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Err(error) => {
            print_argument_error(error);
            ExitCode::FAILURE
        }
    }
}

fn print_host_id() -> ExitCode {
    let path = config_path();
    match read_host_id(&path) {
        Ok(host_id) => {
            println!("{host_id}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("hostid: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_argument_error(error: ArgumentError) {
    match error {
        ArgumentError::ExtraOperand(argument) => {
            eprintln!("hostid: extra operand '{}'", display_argument(&argument));
        }
        ArgumentError::UnknownOption(argument) => {
            eprintln!(
                "hostid: unrecognized option '{}'",
                display_argument(&argument)
            );
        }
    }
    eprintln!("Try 'hostid --help' for more information.");
}

fn display_argument(argument: &OsStr) -> String {
    argument.to_string_lossy().into_owned()
}
