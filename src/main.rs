use std::io;
use crate::askpass::UserInfo;
use crate::error::ErrorKind;
use nix::unistd::{ForkResult, setgid, Gid, initgroups, setuid, Uid, fork};
use users::get_user_by_name;
use std::ffi::CString;
use std::env::set_current_dir;
use crate::login::authenticate;
use crate::x::start_x;
use std::path::Path;

pub mod askpass;
pub mod error;
pub mod login;
pub mod x;

fn main() -> io::Result<()>{

    let tty = 2;

    // de-hardcode 2
    match chvt::chvt(tty) {
        Ok(_) => (),
        Err(_) => {
            println!("Could not change console");
        }
    };

    let mut auth: Result<UserInfo, ErrorKind>;

    loop {
        auth = authenticate();

        if auth.is_ok() {
            break;
        }
    }

    // Safe because we check is_ok()
    let user_info = auth.unwrap();

    match fork() {
        Ok(ForkResult::Child) => {

            println!("Logged in as: {}", std::env::var("USER").unwrap());
            println!("Current directory: {}", std::env::var("PWD").unwrap());

            let homedir = std::env::var("HOME").unwrap();
            println!("Home directory: {}", &homedir);

            let user= get_user_by_name(&user_info.username).expect("Couldn't find username");

            println!("user: {:?}", user);
            println!("user id: {:?}", user.uid());
            println!("primary group: {:?}", user.primary_group_id());


            setgid(Gid::from_raw(user.primary_group_id())).expect("Could not set GID for your user");

            initgroups(
                &CString::new(user_info.username).unwrap(),
                Gid::from_raw(user.primary_group_id())
            ).expect("Could not assign groups to your user");

            // No Root :(
            setuid(Uid::from_raw(user.uid())).expect("Could not set UID for your user");

            set_current_dir(&homedir).expect("Couldn't set home directory");

            start_x(tty as u32, Path::new(&homedir)).map_err(|e| ErrorKind::XError(e)).expect("Couldn't start X");
        }
        _ => {
            loop {}
        }
    }


    // ask for user / pass
    // authenticate with pam
    // setuid to user
    // startx
    Ok(())

}
