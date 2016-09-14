#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        let _ = write!(&mut ::std::io::stderr(), "\x1b[1;31mERROR:\x1b[0m ");
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)*);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        let _ = write!(&mut ::std::io::stderr(), "\x1b[1;33mWARNING:\x1b[0m ");
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)*);
    }
}
