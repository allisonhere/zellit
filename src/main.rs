mod bundled_themes;
mod config;
mod theme;
mod ui;
mod update;
mod zjstatus;

use ui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    ui::run(app)?;
    Ok(())
}
