mod raw;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Player: std::fmt::Display + Sized {
    fn find_all() -> Vec<Self>;
    fn queue(&self, url: String) -> Result<()>;
}
