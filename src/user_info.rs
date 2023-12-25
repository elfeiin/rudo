use std::path::PathBuf;

use libc::gethostname;
use users::{get_current_gid, get_current_username, Group};
use which::which;

use crate::error::RudoError;

pub fn get_command_path(command: &str) -> Result<PathBuf, RudoError> {
    if let Ok(path) = which(command) {
        Ok(path)
    } else {
        let mut path = PathBuf::new();
        path.push(command.to_owned());
        Ok(path)
    }
}

pub fn get_username() -> Result<String, RudoError> {
    if let Some(username) = get_current_username() {
        if let Ok(username) = username.into_string() {
            Ok(username)
        } else {
            Err(RudoError::BadUsername)
        }
    } else {
        Err(RudoError::BadUsername)
    }
}

pub fn get_groups() -> Vec<Group> {
    if let Some(groups) =
        users::get_user_groups(&get_username().unwrap_or_default(), get_current_gid())
    {
        groups
    } else {
        vec![]
    }
}

pub fn get_hostname() -> Result<String, RudoError> {
    let mut hostname = [0u8; 255];
    unsafe {
        if gethostname(hostname.as_mut_ptr() as *mut i8, hostname.len()) != 0 {
            return Err(RudoError::BadHostname);
        }
    };
    if let Ok(s) = String::from_utf8(hostname.to_vec()) {
        let s = s.trim_matches(char::from_u32(0).unwrap()).to_owned();
        Ok(s)
    } else {
        Err(RudoError::BadHostname)
    }
}
