use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const CONFIG_ENV: &str = "MYHOSTID_CONFIG";
pub const DEFAULT_CONFIG_PATH: &str = "/etc/myhostid.conf";

#[derive(Debug)]
pub enum ConfigError {
    Read { path: PathBuf, source: io::Error },
    Empty { path: PathBuf },
    MultipleLines { path: PathBuf },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(f, "cannot read '{}': {source}", path.display())
            }
            Self::Empty { path } => {
                write!(f, "configuration file '{}' is empty", path.display())
            }
            Self::MultipleLines { path } => write!(
                f,
                "configuration file '{}' must contain exactly one line",
                path.display()
            ),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Empty { .. } | Self::MultipleLines { .. } => None,
        }
    }
}

pub fn config_path() -> PathBuf {
    std::env::var_os(CONFIG_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH))
}

pub fn read_host_id(path: &Path) -> Result<String, ConfigError> {
    let contents = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_owned(),
        source,
    })?;
    parse_host_id(path, &contents)
}

fn parse_host_id(path: &Path, contents: &str) -> Result<String, ConfigError> {
    let value = contents.trim();

    if value.is_empty() {
        return Err(ConfigError::Empty {
            path: path.to_owned(),
        });
    }

    if value.contains(['\n', '\r']) {
        return Err(ConfigError::MultipleLines {
            path: path.to_owned(),
        });
    }

    Ok(value.to_owned())
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    PrintHostId,
    Help,
    Version,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArgumentError {
    ExtraOperand(OsString),
    UnknownOption(OsString),
}

pub fn parse_args<I>(args: I) -> Result<Action, ArgumentError>
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let Some(argument) = args.next() else {
        return Ok(Action::PrintHostId);
    };

    if argument == OsStr::new("--help") {
        return Ok(Action::Help);
    }
    if argument == OsStr::new("--version") {
        return Ok(Action::Version);
    }

    if argument.to_string_lossy().starts_with('-') {
        Err(ArgumentError::UnknownOption(argument))
    } else {
        Err(ArgumentError::ExtraOperand(argument))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_single_value_with_surrounding_whitespace() {
        let value = parse_host_id(Path::new("config"), "  BED0745A0764\r\n").unwrap();
        assert_eq!(value, "BED0745A0764");
    }

    #[test]
    fn does_not_force_linux_hostid_length_or_case() {
        let value = parse_host_id(Path::new("config"), "BED0745A0764").unwrap();
        assert_eq!(value, "BED0745A0764");
    }

    #[test]
    fn rejects_empty_configuration() {
        assert!(matches!(
            parse_host_id(Path::new("config"), " \n"),
            Err(ConfigError::Empty { .. })
        ));
    }

    #[test]
    fn rejects_more_than_one_line() {
        assert!(matches!(
            parse_host_id(Path::new("config"), "first\nsecond\n"),
            Err(ConfigError::MultipleLines { .. })
        ));
    }

    #[test]
    fn parses_supported_arguments() {
        assert_eq!(parse_args([]).unwrap(), Action::PrintHostId);
        assert_eq!(
            parse_args([OsString::from("--help")]).unwrap(),
            Action::Help
        );
        assert_eq!(
            parse_args([OsString::from("--version")]).unwrap(),
            Action::Version
        );
    }
}
