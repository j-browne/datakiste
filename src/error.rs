#![allow(deprecated)]
error_chain! {
    foreign_links{
        Int(std::num::ParseIntError);
        Float(std::num::ParseFloatError);
        Io(std::io::Error) #[cfg(unix)];
    }
}
