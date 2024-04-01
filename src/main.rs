use antology::*;

fn try_main() -> anyhow::Result<()> {
    let (event_loop, window) = init()?;
    pollster::block_on(run(event_loop, window))?;

    Ok(())
}

fn main() {
    env_logger::init();

    match try_main() {
        Ok(()) => log::info!("Exiting gracefully"),
        Err(e) => log::error!("Error occured: {}", e),
    };
}
