use witfun;

/// Sample function. This function will read one argument passed into it. Interpret it as UTF-8,
/// and return capitalized version of the input data.
#[witfun::entrypoint]
pub fn capitalize() {
    let v1 = witfun::arg_buf(0);
    let mut v = String::from_utf8_lossy(&v1).to_string();
    v.make_ascii_uppercase();
    witfun::ret(&v.as_bytes());
}
