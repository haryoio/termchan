use eyre::Result;
use keyring;

const KEYRING_SERVICE: &str = "termchan";
const KEYRING_USER: &str = "termchan";

pub fn set_password(service: &str, account: &str, password: &str) -> Result<()> {
    Ok(())
}

pub fn get_account_password(service: &str, account: &str) -> Result<()> {
    Ok(())
}
