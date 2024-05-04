use async_trait::async_trait;
use engine::application::scene::PrefabId;
use engine::resources::model::ModelId;
use engine::systems::network::InternalSender;
use engine::systems::Backpack;
use engine::{
  application::{
    downloader::DownloadSender,
    gamefile::Gamefile,
    input::TrustedInput,
    scene::{Prefab, Scene},
  },
  networking::connection::{PlayerId, Protocol},
  systems::{
    network::{ChannelEvents, ClientSender},
    trusty::ServerControls,
    Initializable, Inventory,
  },
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
enum ModelNames {
  Player,
}

#[allow(dead_code)]
pub struct NetworkController {
  prefabs: HashMap<ModelNames, Prefab>,
  enemy_prefabs: HashMap<PrefabId, Prefab>,
  download_sender: DownloadSender,
  server: InternalSender<ServerControls>,
  client_sender: ClientSender<TrustedInput>,
  bunny_models: Vec<ModelId>,
  bunny_models_available: Vec<ModelId>,
  bunny_models_in_use: HashMap<PlayerId, ModelId>,
}

impl Initializable for NetworkController {
  fn initialize(inventory: &Inventory) -> Self {
    let download_sender = inventory.get::<DownloadSender>().clone();
    let client_sender = inventory.get::<ClientSender<TrustedInput>>().clone();
    let server = inventory.get::<InternalSender<ServerControls>>().clone();
    Self {
      client_sender,
      download_sender,
      server,
      prefabs: HashMap::new(),
      enemy_prefabs: HashMap::new(),
      bunny_models: Vec::new(),
      bunny_models_available: Vec::new(),
      bunny_models_in_use: HashMap::new(),
    }
  }
}

#[async_trait]
impl ChannelEvents for NetworkController {
  fn on_session_start(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let gamefile = Gamefile::from_file(&self.download_sender, "moon.lvl");
    scene.load_level(&gamefile);

    for (_id, prefab) in gamefile.scene.prefabs {
      match prefab.tag.name.as_str() {
        "Player" => {
          log::info!("creating player prefab: {:?}", prefab.tag.name);
          self.prefabs.insert(ModelNames::Player, prefab.clone());
        }
        _ => todo!(),
      }
    }

    backpack.insert::<HashMap<PrefabId, Prefab>>(self.enemy_prefabs.clone());
  }

  fn on_player_joined(
    &mut self,
    _: &mut Scene,
    _: &mut Backpack,
    _: &HashSet<PlayerId>,
    player_id: PlayerId,
    _: String,
    protocol: Protocol,
  ) {
    if protocol != Protocol::Tcp {
      return;
    }

    self.server.send(ServerControls::SyncWorld { player_id });
  }

  fn on_player_left(
    &mut self,
    _scene: &mut Scene,
    _backpack: &mut Backpack,
    player_id: PlayerId,
    _protocol: Protocol,
  ) {

    self
      .server
      .send(ServerControls::DisconnectPlayer { player_id });
  }
}

