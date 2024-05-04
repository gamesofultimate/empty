mod camera;

use engine::systems::hdr::HdrMultiplayerPipeline;
use engine::systems::Scheduler;
use engine::utils::browser::grow_memory;
use engine::application::bus::BrowserBus;
use engine::application::input::DefaultInput;


// 1080p
const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;
const FRAMES_PER_SECOND: u64 = 60;
const GROW_MEMORY_IN_MB: u32 = 800;

pub async fn main(
  canvas_id: String,
  assets_location: String,
  _: BrowserBus,
  session_id: String,
  access_token: String,
  udp_url: String,
  tcp_url: String,
) {
  wasm_logger::init(wasm_logger::Config::default());
  grow_memory(GROW_MEMORY_IN_MB);
  let mut runner = Scheduler::new(FRAMES_PER_SECOND, canvas_id);

  log::debug!("assets location: {:?}", &assets_location);

  let hdr = HdrMultiplayerPipeline::<DefaultInput>::new(
    assets_location,
    session_id,
    access_token,
    udp_url,
    tcp_url,
  );

  runner.attach_plugin(hdr);
  runner.run().await;
}
