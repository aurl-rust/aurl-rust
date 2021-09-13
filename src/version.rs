pub fn name() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("aurl-rust {:}", version)
}
