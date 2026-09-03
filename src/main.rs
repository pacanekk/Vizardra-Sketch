mod app;
mod core;
mod renderer;
mod ui;

fn main() {
    let result = app::run();
    if let Err(e) = result {
        eprintln!("Vizardra error: {}", e);
        std::process::exit(1);
    }
}
