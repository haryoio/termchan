pub mod error;
pub mod get;
pub mod header;
pub mod post;
pub mod url;
pub mod util;
pub mod client {

    use crate::{
        get::{bbsmenu::Bbsmenu, board::Board, thread::Thread},
        post::{reply::post_reply, thread::create_thread},
    };
    struct Client {}
    pub trait ClientTrait {
        fn bbsmenu(menu_url: &str) -> Bbsmenu;
        fn board(board_url: &str) -> Board;
        fn thread(thread_url: &str) -> Thread;
    }
    impl ClientTrait for Client {
        fn bbsmenu(url: &str) -> Bbsmenu {
            Bbsmenu::new(url.to_string())
        }
        fn board(board_url: &str) -> Board {
            Board::new(board_url.to_string())
        }
        fn thread(thread_url: &str) -> Thread {
            Thread::new(thread_url.to_string()).unwrap()
        }
    }

    impl Client {
        pub async fn login() -> Self {
            Self {}
        }
    }

    impl Board {
        pub async fn post(
            &self,
            subject: &str,
            message: &str,
            name: Option<&str>,
            mail: Option<&str>,
        ) -> anyhow::Result<String> {
            create_thread(&self.url.clone(), subject, message, name, mail).await
        }
    }
    impl Thread {
        pub async fn post(
            &self,
            message: &str,
            name: Option<&str>,
            mail: Option<&str>,
        ) -> anyhow::Result<String> {
            post_reply(&self.url.clone(), message, name, mail).await
        }
    }
}
