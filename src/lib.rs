// =================================

pub mod macros;

// =================================

pub fn delay(millis: u64) {
    std::thread::sleep(
        std::time::Duration::from_millis(millis)
    );
}

pub fn pause() {
    crate::printf!("Press [Enter] to continue...");
    let _ = crate::input!("");
}

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