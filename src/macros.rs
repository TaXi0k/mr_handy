#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        use std::io::Write;
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! input {
    () => {{
        $crate::printf!(">> ");

        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_ok() {
            buf
        } else {
            String::new()
        }
    }};

    ($($arg:tt)*) => {{
        $crate::printf!($($arg)*);

        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_ok() {
            buf
        } else {
            String::new()
        }
    }};
}

#[macro_export]
macro_rules! try_input {
    () => {{
        $crate::printf!(">> ");

        let mut buf = String::new();
        Ok( std::io::stdin().read_line(&mut buf)? )
    }};

    ($($arg:tt)*) => {{
        $crate::printf!($($arg)*);

        let mut buf = String::new();
        Ok( std::io::stdin().read_line(&mut buf)? )
    }};
}
