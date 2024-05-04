mod network_controller;

use engine::systems::hdr::HdrPipeline;
use crate::server::network_controller::NetworkController;
use engine::systems::Scheduler;

const FRAMES_PER_SECOND: u64 = 60;

pub async fn main() {
  dotenv::dotenv().ok();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  log::info!("Number of cpus: {:}", num_cpus::get());

  let rpc_address = {
    let address = dotenv::var("RPC_ADDRESS").unwrap();
    let port = dotenv::var("RPC_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let session_address = {
    let address = dotenv::var("GAME_ADDRESS").unwrap();
    let port = dotenv::var("GAME_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let (hdr, _) = HdrPipeline::<NetworkController>::new("resources", rpc_address, session_address);
  let mut runner = Scheduler::new(FRAMES_PER_SECOND);
  runner.attach_plugin(hdr);

  runner.run().await;
}
