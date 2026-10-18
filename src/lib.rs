//! # Mr. Handy
//! 
//! A small and simple utility crate made by me for me, but if you like it - feel free to use it just remember to stick to the [LICENSE](https://github.com/TaXi0k/mr_handy/blob/master/LICENSE).

// =================================

mod macros;

// =================================

/// Sleeps the current thread for the specified duration.
/// 
/// A thin wrapper around `std::thread::sleep` to save a few boilerplate keystrokes.
pub fn delay(millis: u64) {
    std::thread::sleep(
        std::time::Duration::from_millis(millis)
    );
}

/// Pauses execution until the user presses Enter.
pub fn pause() {
    let _ = crate::input!("Press [Enter] to continue... ");
}


/// Prompts the user with a yes/no question and waits for confirmation.
/// 
/// # Arguments
/// 
/// * `prompt` - The question or message to display to the user.
/// * `default` - Specifies the behavior when the user hits Enter without typing:
///   * `Some(true)` - Defaults to `true` (displays `[Y/n]`).
///   * `Some(false)` - Defaults to `false` (displays `[y/N]`).
///   * `None` - No default; forces the user to provide a valid answer (`[y/n]`).
/// 
/// # Returns
/// 
/// * `bool` - Returns `true` for affirmative input (`y`, `yes`), `false` for negative input (`n`, `no`), or the provided `default` value if the user hits Enter with empty input.
/// 
/// # Examples
/// 
/// ```no_run
/// // Prompts "Are you sure? [Y/n]: "
/// if confirm("Are you sure?", Some(true)) {
///     println!("Action executed!");
/// }
/// ```
pub fn confirm(prompt: &str, default: Option<bool>) -> bool {

    let (indicator, default_value) = match default {
        Some(true) => ("[Y/n]", true),
        Some(false) => ("[y/N]", false),
        None => ("[y/n]", false),
    };

    loop {
        let response = crate::input!("{} {}: ", prompt, indicator);
        let trimmed = response.trim().to_lowercase();

        if trimmed.is_empty() && default.is_some() {
            return default_value
        }

        match trimmed.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'.")
        }
    }
}