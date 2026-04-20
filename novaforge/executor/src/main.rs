//! Standalone game runner (executor) for NovaForge.
//!
//! Run with:
//!   cargo run --package executor --release

use fyrox::core::log::Log;
use fyrox::engine::executor::Executor;
use fyrox::event_loop::EventLoop;
use novaforge::Game;

fn main() {
    Log::set_file_name("novaforge.log");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut executor = Executor::new(Some(event_loop));
    executor.add_plugin(Game::default());
    executor.run()
}
