/// Prints formatted data to stdout and flushes the stream immediately
/// 
/// Combines `print!` and `.flush()` to ensure output renders instantly 
/// without waiting for a newline - essential for inline prompts.
/// 
/// Intended to be `println!` without the newline at the end.
#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        use std::io::Write;
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
}

/// Reads a line from standard input into a `String`.
/// 
/// Intended to work pretty much like Python's `input()`.
/// 
/// ## Behavior & Arguments
/// * **No arguments:** Automatically prints a default prompt (`>> `).
/// * **Format arguments:** Accepts any standard formatting expressions (like `println!`), printing them as a custom prompt before reading input.
///   * **Empty prompt:** Pass `""` if you want to read input with zero preceding text.
/// 
/// ## Error Handling
/// This is the *"lossy"* variant. If an I/O error occurs while reading from stdin, 
/// it safely swallows the error and returns an empty `String` rather than panicking or failing.
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

/// Reads a line from standard input into a `String`, returning an `Result<String, std::io::Error>`.
/// 
/// Intended to work pretty much like Python's `input()` but with errors.
/// 
/// ## Behavior & Arguments
/// * **No arguments:** Automatically prints a default prompt (`>> `) using [`printf!`].
/// * **Format arguments:** Accepts any standard formatting expressions (like `println!`), printing them as a custom prompt before reading input.
///   * **Empty prompt:** Pass `""` if you want to read input with zero preceding text.
/// 
/// ## Error Handling
/// This is the strict variant. It propagates I/O errors, returning an `Result<String, std::io::Error>`, allowing you to handle or propagate errors yourself.
#[macro_export]
macro_rules! try_input {
    () => {{
        $crate::printf!(">> ");

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        Ok( buf )
    }};

    ($($arg:tt)*) => {{
        $crate::printf!($($arg)*);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        Ok( buf )
    }};
}
