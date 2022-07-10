pub mod url {
    pub mod reply;
    pub mod thread;
    pub mod url;
}
pub mod util {
    pub mod encoding;
    pub mod error;
    pub mod time;
}
pub mod header {
    pub mod build;
    pub mod cookie;
}
pub mod get {
    pub mod bbsmenu;
    pub mod board;
    pub mod board_cert;
    pub mod thread;
}
pub mod post {
    pub mod reply;
    pub mod reply_with_login;
    pub mod thread;
    pub mod thread_with_login;
    pub mod form {
        pub mod reply;
        pub mod thread;
    }
}
