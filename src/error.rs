use std::fmt::Display;

#[derive(Debug)]
pub enum RudoError {
    AuthenticationError,
    NoCommandSpecified,
    CommandNotFound,
    PermissionDenied,
    MalformedRudoersToml,
    NotInRudoers,
    NoSwitchEntry,
    UnknownError,
    BadHostname,
    BadUsername,
    BadGroupname,
}

impl Display for RudoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &str = match self {
            RudoError::AuthenticationError => "incorrect password".into(),
            RudoError::NoCommandSpecified => "no command specified".into(),
            RudoError::PermissionDenied => "permission denied :)".into(),
            RudoError::MalformedRudoersToml => "malformed rudoers TODO: display error span".into(),
            RudoError::NotInRudoers => "user is not in the rudoers file! >:C".into(),
            RudoError::NoSwitchEntry => "not allowed to run as that user! >:C".into(),
            RudoError::UnknownError => "unknown error".into(),
            RudoError::BadHostname => "bad hostname".into(),
            RudoError::BadUsername => "bad username".into(),
            RudoError::BadGroupname => "bad groupname".into(),
            RudoError::CommandNotFound => "command not found".into(),
        };
        writeln![f, "{text}"]
    }
}
