mod autocomplete;
pub mod parser;
pub use parser::Caprice;

pub(crate) type Result<T> = std::result::Result<T, std::io::Error>;
