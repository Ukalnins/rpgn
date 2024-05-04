use std::{io::{Error, Read, Write}, net::TcpStream};
use std::collections::HashMap;

pub enum Address {
    IP4(String, u32),
    #[cfg(target_os = "linux")]
    Socket(String),
}

trait RWTrait : Read + Write {}
impl RWTrait for TcpStream {}

#[cfg(target_os = "linux")]
mod linux_only {
    use std::{io::Error, os::unix::net::UnixStream};

    impl crate::pg::connection::RWTrait for UnixStream {}

    pub fn open_socket(path: &str) -> Result<UnixStream, Error> {
        UnixStream::connect(path)
    }
}


pub struct ConnectionInfo {
    pub addr: Address,
    pub user: String,
    pub db: String,
}

pub struct PgCon {
    stream: Box<dyn RWTrait>,
}


struct StartupMsg<'a> {
    vers: i32, // 196608
    params: &'a HashMap<&'a str, &'a str>,
}

trait Msg {

}


impl Msg for StartupMsg<'_> {

}

impl PgCon {
    pub fn new(con_info: &ConnectionInfo) -> Result<Self, Error> {
        let con : Box<dyn RWTrait> = match &con_info.addr {
            Address::IP4( host, port) => {
                match TcpStream::connect(format!("{host}:{port}")) {
                    Ok(stream) => Box::new(stream),
                    Err(e) => return Err(e),
                }
            },
            #[cfg(target_os = "linux")]
            Address::Socket(path) => {
                match linux_only::open_socket(&path) {
                    Ok(stream) => Box::new(stream),
                    Err(e) => return Err(e),
                }
            }
        };
        let con = PgCon{stream : con};
        let mut startup_params : HashMap<&str, &str> = HashMap::new();
        startup_params.insert("user", &con_info.user);
        con.send_message(StartupMsg{vers: 196608, params: &startup_params});

        Ok(con)
    }

    fn send_message<M: Msg>(self, msg: M) -> Result<(), Error> {
        Ok(())
    }
}