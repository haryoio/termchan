use std::error::Error;

use termchan_tui::run::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}
