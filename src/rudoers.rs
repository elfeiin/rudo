use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use users::Group;

use crate::{app::App, user_info::*};
use crate::{error::RudoError, DEFAULT_RUDOERS};

const ALL: &'static str = "ALL";

fn default_password_required() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostPerms {
    #[serde(default = "default_password_required")]
    password_required: bool,
    aliases: BTreeSet<String>,
    #[serde(default)]
    commands: BTreeSet<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Entry {
    CommandPaths(BTreeSet<String>),
    Hosts(BTreeMap<String, HostPerms>),
}

#[derive(Default, Deserialize, Serialize)]
pub struct Rudoers {
    users: BTreeMap<String, Entry>,
    #[serde(default)]
    groups: BTreeMap<String, Entry>,
}

fn parse_rudoers() -> Rudoers {
    let mut rudoers = match File::open(DEFAULT_RUDOERS) {
        Ok(file) => file,
        Err(e) => {
            eprintln!["Cannot open {DEFAULT_RUDOERS}: {e}"];
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    match rudoers.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!["Could not read {DEFAULT_RUDOERS}: {e}"];
            std::process::exit(1);
        }
    };
    match toml::from_str(&contents) {
        Ok(rudoers) => rudoers,
        Err(e) => {
            eprintln!["Failed to parse {DEFAULT_RUDOERS}: {e}"];
            std::process::exit(1);
        }
    }
}

fn check_entry(
    entry: &Entry,
    hostname: &str,
    alias: &str,
    command_path: &str,
) -> Result<bool, RudoError> {
    match entry {
        Entry::CommandPaths(cmds) => {
            if cmds.contains(command_path) || cmds.contains(ALL) {
                Ok(true)
            } else {
                Err(RudoError::PermissionDenied)
            }
        }
        Entry::Hosts(hosts) => {
            if let Some(ref host) = hosts.get(hostname).or_else(|| hosts.get(ALL)) {
                if host.aliases.contains(alias) || host.aliases.contains(ALL) {
                    if host.commands.contains(command_path) || host.commands.contains(ALL) {
                        Ok(host.password_required)
                    } else {
                        Err(RudoError::PermissionDenied)
                    }
                } else {
                    Err(if alias == "root" {
                        RudoError::PermissionDenied
                    } else {
                        RudoError::NoSwitchEntry
                    })
                }
            } else {
                Err(RudoError::PermissionDenied)
            }
        }
    }
}

pub fn check_rudoers(args: &App) -> Result<bool, RudoError> {
    let command_path: PathBuf = if let Some(ref cmd) = args.cmd {
        get_command_path(cmd.first().unwrap())?
    } else {
        return Err(RudoError::NoCommandSpecified);
    };
    let alias: String = args.user.clone().unwrap_or("root".into());
    let hostname: String = get_hostname()?;
    let username: String = get_username()?;
    let groups: Vec<Group> = get_groups();
    let rudoers = parse_rudoers();
    Ok(if let Some(entry) = rudoers.users.get(&username) {
        check_entry(&entry, &hostname, &alias, &command_path.to_string_lossy())?
    } else {
        let mut result = Err(RudoError::NotInRudoers);
        for group in groups {
            if let Some(entry) = rudoers.groups.get(group.name().to_str().unwrap()) {
                let output =
                    check_entry(&entry, &hostname, &alias, &command_path.to_string_lossy());
                if output.is_ok() {
                    result = output;
                    break;
                }
            }
        }
        result?
    })
}

#[test]
fn rudoers_toml() {
    let mut rudoers = Rudoers::default();
    rudoers.users.insert(
        "alice".to_string(),
        Entry::Hosts({
            let mut hosts = BTreeMap::new();
            hosts.insert(
                "northpole".to_string(),
                HostPerms {
                    password_required: false,
                    aliases: {
                        let mut set = BTreeSet::new();
                        set.insert("root".to_string());
                        set
                    },
                    commands: {
                        let mut commands = BTreeSet::new();
                        commands.insert("/bin/cat".into());
                        commands.insert("/bin/grep".into());
                        commands.insert("/bin/xargs".into());
                        commands
                    },
                },
            );
            hosts
        }),
    );
    rudoers.users.insert(
        "bob".to_string(),
        Entry::Hosts({
            let mut hosts = BTreeMap::new();
            hosts.insert(
                "northpole".to_string(),
                HostPerms {
                    password_required: false,
                    aliases: {
                        let mut set = BTreeSet::new();
                        set.insert("root".to_string());
                        set
                    },
                    commands: {
                        let mut commands = BTreeSet::new();
                        commands.insert("/bin/cat".into());
                        commands.insert("/bin/grep".into());
                        commands.insert("/bin/xargs".into());
                        commands
                    },
                },
            );
            hosts
        }),
    );
    let rudoers = toml::to_string(&rudoers).unwrap();
    let mut text = String::new();
    for (n, s) in rudoers.lines().enumerate() {
        if n > 0 {
            text.push('\n');
        }
        if s.starts_with(|c: char| !c.is_whitespace()) {
            text.push_str(&format!["# {s}"]);
        } else {
            text.push_str(&s);
        }
    }
    text.push('\n');
    File::create("test_config")
        .unwrap()
        .write_all(&text.into_bytes())
        .unwrap();
}
