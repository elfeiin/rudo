use crate::{error::RudoError, user_info::get_username, AUTH_SERVICE, RETRIES};

pub fn check_auth() -> Result<(), RudoError> {
    let login = get_username().unwrap();
    let mut retries = RETRIES;
    for i in 0..retries {
        let password = rpassword::prompt_password(format!["password for {login}: "]).unwrap();
        let mut auth = pam::Authenticator::with_password(AUTH_SERVICE).unwrap();
        auth.get_handler().set_credentials(&login, password);
        match auth.authenticate() {
            Ok(()) => return Ok(()),
            Err(_) => {
                if i < retries - 1 {
                    eprintln!["incorrect password, try again"]
                }
            }
        }
    }
    Err(RudoError::AuthenticationError)
}
