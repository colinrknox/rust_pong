use macroquad::window::Conf;

#[macroquad::main(config)]
async fn main() {
    rust_pong::macroquad_run().await;
}

fn config() -> Conf {
    rust_pong::config()
}
