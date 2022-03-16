use reqwest::Response as ReqwestResult;

#[derive(Debug)]
pub enum Response {
    Dispatched(ReqwestResult),
    SnippetGenerated(String),
}

impl From<ReqwestResult> for Response {
    fn from(r: ReqwestResult) -> Self {
        Response::Dispatched(r)
    }
}
