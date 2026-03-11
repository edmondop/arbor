/// Extension trait that adds `.context()` to any `Result` type, converting
/// the error into a `String` with additional context. This is a lightweight
/// alternative to `anyhow::Context` for crates that use string-based errors.
pub trait ResultExt<T> {
    /// Wrap the error with additional context.
    fn context(self, msg: &str) -> Result<T, String>;

    /// Wrap the error with a lazily-evaluated context message.
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for Result<T, E> {
    fn context(self, msg: &str) -> Result<T, String> {
        self.map_err(|e| format!("{msg}: {e}"))
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, String> {
        self.map_err(|e| format!("{}: {e}", f()))
    }
}

/// Extension trait that adds `.context()` to `Option`, converting
/// `None` into a `String` error.
pub trait OptionExt<T> {
    fn context(self, msg: &str) -> Result<T, String>;

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, String>;
}

impl<T> OptionExt<T> for Option<T> {
    fn context(self, msg: &str) -> Result<T, String> {
        self.ok_or_else(|| msg.to_owned())
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, String> {
        self.ok_or_else(f)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn result_context_wraps_error_message() {
        let err: Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "gone"));
        let result = err.context("reading config");
        assert_eq!(result.unwrap_err(), "reading config: gone");
    }

    #[test]
    fn result_with_context_uses_lazy_message() {
        let err: Result<(), &str> = Err("timeout");
        let path = "/tmp/foo";
        let result = err.with_context(|| format!("loading {path}"));
        assert_eq!(result.unwrap_err(), "loading /tmp/foo: timeout");
    }

    #[test]
    fn option_context_converts_none_to_error() {
        let opt: Option<u32> = None;
        let result = opt.context("missing value");
        assert_eq!(result.unwrap_err(), "missing value");
    }

    #[test]
    fn option_context_passes_through_some() {
        let opt = Some(42);
        assert_eq!(opt.context("missing").expect("should be Ok"), 42);
    }
}
