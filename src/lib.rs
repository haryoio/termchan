use anyhow::{Context, Ok, Result};
pub mod utils {
    pub mod encoder;
    pub mod headers;
    pub mod pattterns;
    pub mod receiver;
    pub mod sender;
}
pub mod controller {
    pub mod board;
    pub mod reply;
    pub mod thread;
}
