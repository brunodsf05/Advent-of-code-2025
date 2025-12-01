use std::fs;
use std::process::exit;

pub fn quick_read_impl(base: &str, path: &str) -> String {
    let full_path = std::path::Path::new(base).join(path);

    match fs::read_to_string(&full_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Couldn't read {full_path:?}.\n{e:?}");
            exit(-1);
        }
    }
}

/// Reads a file, if there is an error, the program closes.
/// The path is relative to the proyect's `Cargo.toml` file. That means you can
/// reference the file relatively from a proyect and not from workspace's CWD.
#[macro_export]
macro_rules! quick_read {
    ($path:expr) => {{ $crate::quick_read_impl(env!("CARGO_MANIFEST_DIR"), $path) }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_read_hello() {
        let result = quick_read!("assets/hello.txt");
        assert_eq!(result.trim(), "Hello world!");
    }
}
