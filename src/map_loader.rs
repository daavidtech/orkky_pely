use std::path::Path;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use bevy::prelude::Resource;
use notify::WatcherKind::ReadDirectoryChangesWatcher;
use notify::RecursiveMode;
use notify::Watcher;

use crate::map::Map;
use crate::map::MapChange;
use crate::map::MapChanges;

// fn worker(path: &str) -> ReadDirectoryChangesWatcher {
// 	println!("starting map loader worker {}", path);

// 	let (tx, rx) = std::sync::mpsc::channel();

// 	let mut watcher = notify::recommended_watcher(tx).unwrap();

// 	|res| {
//         match res {
//            Ok(event) => { 
// 				println!("event: {:?}", event)
// 		   },
//            Err(e) => println!("watch error: {:?}", e),
//         }
//     };

// 	println!("adding path to watcher: {}", path);

//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(Path::new(path), RecursiveMode::Recursive).unwrap();

// 	watcher
// }

struct Worker {
	watcher: notify::RecommendedWatcher,
}

pub fn create_map_loader(path: &str) -> MapChangesReceiver {
	let (tx, rx) = std::sync::mpsc::channel();

	let path = path.to_string();

	thread::spawn(move || {
		let path = path;

		let map = Map::load(&path);

		if let Some(entities) = map.entities {
			for entity in entities {
				println!("new entity {}", entity.template);
	
				tx.send(MapChange::NewMapEntity(entity));
			}
		}

		if let Some(template) = map.templates {
			for template in template {
				println!("new template {}", template.name);
	
				tx.send(MapChange::NewMapTemplate(template));
			}
		}

		if let Some(shapes) = map.shapes {
			for shape in shapes {
				println!("new shape {:?}", shape);
	
				tx.send(MapChange::NewMapShape(shape));
			}
		}

		if let Some(light) = map.lights {
			for light in light {
				println!("new light {:?}", light);
	
				tx.send(MapChange::NewLight(light));
			}
		}

		if let Some(ambient_light) = map.ambient_light {
			println!("new ambient light {:?}", ambient_light);

			tx.send(MapChange::NewAmbientLight(ambient_light));
		}
	});

	MapChangesReceiver {
		rx: Mutex::new(rx)
	}
}

#[derive(Resource)]
pub struct MapChangesReceiver {
	pub rx: Mutex<mpsc::Receiver<MapChange>>
}

pub struct MapLoader {
	watch_changes: bool,
	path: String,
	map_changes: MapChanges,
	watcher: notify::RecommendedWatcher,
	// last_map: Map
}

// impl MapLoader {
// 	pub fn new(path: &str) -> MapLoader {
// 		let watcher = worker(path);

// 		let mut map_changes = MapChanges::new();

// 		MapLoader {
// 			watch_changes: false,
// 			path: path.to_string(),
// 			map_changes: map_changes,
// 			watcher,
// 			// last_map: Map::load(path)
// 		}
// 	}

// 	pub fn load(&mut self) {
// 		let map = Map::load(&self.path);

// 		for entity in map.entities {
// 			println!("new entity {}", entity.template);

// 			self.map_changes.changes.push(MapChange::NewMapEntity(entity));
// 		}

// 		for template in map.templates {
// 			println!("new template {}", template.name);

// 			self.map_changes.changes.push(MapChange::NewMapTemplate(template));
// 		}
// 	}

// 	pub fn get_map_changes(&self) -> MapChanges {
// 		self.map_changes.clone()
// 	}
// }
