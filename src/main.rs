use std::{
    fs::{DirBuilder, File, OpenOptions},
    io::{Error, Read, Write},
    os::unix::process::CommandExt,
    path::PathBuf,
    process::Command,
    thread,
    time::{Duration, Instant},
};

mod app;
use app::App;
use clap::Parser;
use error::RudoError;
mod error;
mod rudoers;
mod user_info;
use fork::{daemon, Fork};
use libc::kill;
use rudoers::check_rudoers;
mod authentication;
use authentication::check_auth;
use user_info::get_command_path;

/// RUDOLPH THE RED NOSE REINDEER
/// HAD A VERY SHINY NOSE
/// AND IF YOU EVER SAW IT
/// YOU MIGHT EVEN SAY IT GLOWS

const DEFAULT_RUDOERS: &'static str = "/etc/rudoers.toml";
const AUTH_SERVICE: &'static str = "system-auth";
const RETRIES: u32 = 3;
const DEFAULT_AUTH_CACHE_DURATION: u32 = 15 * 60;
const AUTH_CACHE_PATH: &str = "/var/db/rudo/cache/auth/";
const SESSION_CACHE_PATH: &str = "/var/db/rudo/cache/session/";

pub fn update_auth_cache(alias: &str, ppid: &str) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(AUTH_CACHE_PATH);
    path.push(ppid);
    DirBuilder::new().recursive(true).create(&path)?;
    path.push(alias);
    let mut opts = OpenOptions::new();
    opts.write(true).create(true);
    let mut file = opts.open(&path)?;
    let time = Instant::now();
    let time = unsafe { std::mem::transmute::<Instant, u128>(time) };
    file.write(&bincode::serialize(&time).unwrap())?;
    Ok(())
}

pub fn check_auth_cache(alias: &str, ppid: &str) -> bool {
    let mut path = PathBuf::from(AUTH_CACHE_PATH);
    path.push(ppid);
    path.push(alias);
    let mut file = if let Ok(file) = File::open(&path) {
        file
    } else {
        return false;
    };
    let mut buf = Vec::new();
    if let Err(_) = file.read_to_end(&mut buf) {
        return false;
    };
    match bincode::deserialize::<u128>(&buf) {
        Ok(n) => {
            let old_time = unsafe { std::mem::transmute::<u128, Instant>(n) };
            if old_time.elapsed() < Duration::from_secs(DEFAULT_AUTH_CACHE_DURATION as u64) {
                true
            } else {
                false
            }
        }
        Err(e) => {
            dbg!["{}", e];
            false
        }
    }
}

pub fn execute(app: &App) -> Result<(), RudoError> {
    if let Some(ref cmds) = app.cmd {
        let run_as_name = match &app.user {
            Some(x) => &x,
            None => "root",
        };
        let run_as = users::get_user_by_name(run_as_name).unwrap();

        let mut child = Command::new(get_command_path(cmds.first().unwrap())?)
            .uid(run_as.uid())
            .args(&cmds[1..])
            .spawn()
            .unwrap();
        if app.background {
            std::process::exit(0);
        } else {
            child.wait().unwrap();
        }
    } else {
        return Err(RudoError::NoCommandSpecified);
    }
    Ok(())
}

pub fn run() -> Result<(), RudoError> {
    let app = App::parse();
    let alias = match &app.user {
        Some(x) => &x,
        None => "root",
    };

    let parent_id = std::os::unix::process::parent_id();
    let parent_id_str = format!["{parent_id}"];

    let needs_password = check_rudoers(&app)? && !check_auth_cache(alias, &parent_id_str) || app.validate;
    if needs_password {
        check_auth()?;
    }

    if let Err(e) = update_auth_cache(alias, &parent_id_str) {
        eprintln!["Failed to update cache: {e}"];
        std::process::exit(1);
    }

    let mut path = PathBuf::from(SESSION_CACHE_PATH);
    path.push(&parent_id_str);
    match OpenOptions::new().create_new(true).write(true).open(&path) {
        Ok(_) => {
            if let Ok(Fork::Child) = daemon(false, false) {
                loop {
                    if unsafe { kill(parent_id as _, 0) } == -1 {
                        if let Err(e) = std::fs::remove_file(&path) {
                            println!["Failed to remove session file: {e}"];
                            std::process::exit(1);
                        }
                        let mut path = PathBuf::from(AUTH_CACHE_PATH);
                        path.push(parent_id_str);
                        if let Err(e) = std::fs::remove_dir_all(&path) {
                            println!["Failed to remove session dir: {e}"];
                            std::process::exit(1);
                        }
                        break;
                    }
                    thread::sleep(Duration::from_millis(1000))
                }
            }
        }
        Err(_) => (),
    };

    // Run the command
    execute(&app)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!["{e}"],
    }
    Ok(())
}
