use std::str::FromStr;

use reqwest::Request;

#[derive(Debug)]
pub enum Type {
    Curl,
}

impl FromStr for Type {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "curl" => Ok(Type::Curl),
            _ => Err(String::from("UnknownType")),
        }
    }
    type Err = String;
}

pub trait Output {
    fn output(self, req: &Request) -> String;
}

pub struct Curl {}

impl Output for Curl {
    fn output(self, req: &Request) -> String {
        // Desc: 取れる情報を回して curl 文字列を作る
        // -X METHOD
        // -H 可視なヘッダのみ叩き込む
        //    Sensitive になっているヘッダの場合のみ、オブジェクト生成時にいれたヘッダ情報を入力として入れる
        //    基本的に Sensitive なヘッダはSkip（複数ある場合を考慮してないけど、複数入れたほうがいいか
        // -d のボディ突っ込むのは今の所対応しない
        let method = format!("-X {}", String::from(req.method().as_str()));

        let headers = req.headers();
        let mut header_vec = Vec::<String>::new();
        for (k, v) in headers {
            header_vec.push(format!(
                "-H \"{}: {}\"",
                String::from(k.as_str()),
                String::from(v.to_str().unwrap())
            ));
        }
        format!(
            "curl {} {} {}",
            method,
            header_vec.join(" "),
            String::from(req.url().as_str())
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::Client;

    #[test]
    fn export_curl_test_without_authorization() {
        let req = Client::new()
            .request(reqwest::Method::GET, "https://example.com/test")
            .header("key_001", "value")
            .header("key_002", "value")
            .build()
            .unwrap();

        let curl = Curl {};
        let header_str = curl.output(&req);
        assert_eq!(
            "curl -X GET -H \"key_001: value\" -H \"key_002: value\" https://example.com/test",
            header_str
        );
    }

    #[test]
    fn export_header_test_without_basic() {
        // setup
        let req = Client::new()
            .request(reqwest::Method::GET, "https://example.com/test")
            .header("key_001", "value")
            .basic_auth("username", Some("password"))
            .build()
            .unwrap();
        let curl = Curl {};

        // exercise
        let header_str = curl.output(&req);

        // verify
        let auth_value = base64::encode("username:password".as_bytes());
        assert_eq!(format!("curl -X GET -H \"key_001: value\" -H \"authorization: Basic {}\" https://example.com/test", auth_value), header_str);
    }
}
