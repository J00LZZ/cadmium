use crate::askpass::UserInfo;
use std::io;
use std::io::Write;
use rpassword::read_password;

pub fn simple_get_credentials() -> io::Result<UserInfo> {

    println!("Login:");
    print!("username: ");
    io::stdout().flush().ok().expect("Could not flush stdout");


    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    username.truncate(username.trim_end().len());

    print!("password (hidden): ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let password = read_password()?;

    Ok(UserInfo {
        username,
        password
    })
}
