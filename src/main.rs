use std::sync::Arc;

use anyhow::{Context, Result};
use bi_widget_rust_cli::{
    app::{connect_websocket, App},
    tui::{restore_terminal, run, setup_terminal},
};

use dotenv::dotenv;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mut terminal = setup_terminal().context("setup failed")?;
    let mut app = App::new().await;

    app.select_row(0);
    let arc_mut_app = Arc::new(Mutex::new(app));

    let t_app = Arc::clone(&arc_mut_app);
    tokio::spawn(async {
        let _ = connect_websocket(t_app).await;
    });

    // app.set_items(&exchange_info);
    run(&mut terminal, arc_mut_app)
        .await
        .context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
