pub trait ErrAsString<T> {
    fn e(self) -> Result<T, String>;
}

impl<T, E: ToString> ErrAsString<T> for Result<T, E> {
    fn e(self) -> Result<T, String> {
        self.map_err(|s| s.to_string())
    }
}