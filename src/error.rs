#![allow(deprecated)]
error_chain! {
    foreign_links{
        Json(serde_json::Error);
        Int(std::num::ParseIntError);
        Float(std::num::ParseFloatError);
        Io(std::io::Error) #[cfg(unix)];
    }
}
