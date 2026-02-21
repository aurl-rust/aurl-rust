use log::{debug, error};
use std::io::Write;
use std::io::{self, BufRead};
use std::net::TcpListener;

mod path_util {
    use clap::lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref PATH_REGEX: Regex = Regex::new(".*?code=(?P<code>.+)&").unwrap();
    }

    pub fn split_auth_code<'a>(
        first_line: &'a str,
        expected_path: &str,
    ) -> Result<&'a str, &'static str> {
        let mut params = first_line.split_whitespace();
        let method = params.next();
        let path = params.next();

        match (method, path) {
            (Some("GET"), Some(path)) => {
                // Validate the path portion (before ?) matches expected callback path
                let path_only = path.split('?').next().unwrap_or(path);
                if path_only != expected_path {
                    return Err("Unexpected callback path");
                }

                println!("path: {}", path);

                if let Some(code) = PATH_REGEX.captures(path) {
                    let code = code.name("code").unwrap().as_str();
                    Ok(code)
                } else {
                    Err("failed capture auth_code")
                }
            }
            _ => Err("Unknown Http Method."),
        }
    }
}

pub struct AuthCodeServer {
    port: u16,
    path: String,
}

impl AuthCodeServer {
    pub fn new(port: u16, path: String) -> AuthCodeServer {
        AuthCodeServer { port, path }
    }

    pub fn receive_auth_code(self) -> Result<String, String> {
        let server = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        if let Some(stream) = server.incoming().next() {
            match stream {
                Ok(stream) => {
                    let mut stream = io::BufReader::new(stream);
                    let mut first_line = String::new();
                    if let Err(error) = stream.read_line(&mut first_line) {
                        error!("{}", error);
                    }

                    match path_util::split_auth_code(first_line.as_str(), &self.path) {
                        Ok(code) => {
                            let stream = stream.get_mut();
                            writeln!(stream, "HTTP/1.1 200 OK").unwrap();
                            writeln!(stream, "Content-Type: text/plain; charset=UTF-8\r\n")
                                .unwrap();
                            writeln!(stream, "auth_code={}", code).unwrap();

                            debug!("get auth code: {}", code);
                            return Ok(code.to_string());
                        }
                        Err(err) => error!("{}", err),
                    }
                }
                Err(err) => {
                    error!("Server Received Unexpected Response. {}", err);
                }
            }
        }
        Err("Can not receive auth_code".to_string())
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

    #[test]
    fn test_split_auth_code_valid_path() {
        let first_line = "GET /callback?code=ZZZZ-XXXX-CCCC&state=hogehoge HTTP/1.1";
        let result = path_util::split_auth_code(first_line, "/callback");
        assert_eq!(result, Ok("ZZZZ-XXXX-CCCC"));
    }

    #[test]
    fn test_split_auth_code_unexpected_path() {
        let first_line = "GET /other?code=ZZZZ-XXXX-CCCC&state=hogehoge HTTP/1.1";
        let result = path_util::split_auth_code(first_line, "/callback");
        assert!(result.is_err());
    }
}
