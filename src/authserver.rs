use log::{error, debug};
use std::io::Write;
use std::io::{self, BufRead};
use std::net::TcpListener;

mod path_util {
    use clap::lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref PATH_REGEX: Regex = Regex::new(".*?code=(?P<code>.+)&").unwrap();
    }
}

pub struct AuthCodeServer {
    port: i32,
}

impl AuthCodeServer {
    pub fn new(port: i32) -> AuthCodeServer {
        AuthCodeServer { port }
    }

    pub fn receive_auth_code(self) -> Result<String, ()> {
        let server = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        let mut auth_code_opt: Option<String> = None;
        if let Some(stream) = server.incoming().next() {
            match stream {
                Ok(stream) => {
                    let mut stream = io::BufReader::new(stream);
                    let mut first_line = String::new();
                    if let Err(error) = stream.read_line(&mut first_line) {
                        error!("{}", error);
                    }

                    // スペースで分割
                    let mut params = first_line.split_whitespace();
                    let method = params.next();
                    let path = params.next();
                    let auth_code_result = match (method, path) {
                        (Some("GET"), Some(path)) => {
                            let stream = stream.get_mut();
                            println!("path: {}", path);

                            if let Some(code) = path_util::PATH_REGEX.captures(path) {
                                let code = code.name("code").unwrap().as_str();

                                // ブラウザに空レスポンス
                                writeln!(stream, "HTTP/1.1 200 OK").unwrap();
                                writeln!(stream, "Content-Type: text/plain; charset=UTF-8")
                                    .unwrap();
                                writeln!(stream, "{}", code).unwrap();
                                Ok(code)
                            } else {
                                Err("failed capture auth_code")
                            }
                        }
                        _ => Err("unknown Method"),
                    };
                    match auth_code_result {
                        Ok(code) => auth_code_opt = Some(code.to_string()),
                        Err(str) => error!("{}", str),
                    }
                }
                Err(_) => {
                    error!("Callback Server error");
                }
            }
        }
        match auth_code_opt {
            Some(code) => {
                debug!("get auth code: {}", code);
                Ok(code)
            },
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::path_util;

    #[test]
    fn test_regex_path() {
        match path_util::PATH_REGEX
            .captures("?hogehoge=fugafuga&code=ZZZZ-XXXX-CCCC&state=hogehoge")
        {
            Some(path) => {
                let code = path.name("code").unwrap().as_str();
                assert_eq!(code, "ZZZZ-XXXX-CCCC")
            }
            None => panic!("test failed"),
        }
    }
}
