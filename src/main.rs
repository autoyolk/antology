use antology::*;

fn main() {
    env_logger::init();

    match run() {
        Ok(()) => log::info!("Exiting gracefully"),
        Err(e) => log::error!("Error occured: {}", e),
    };
}
