use std::collections::HashSet;
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

		if let Some(mut entities) = map.entities {
			let mut used_ids = HashSet::new();

			entities.iter().for_each(|e| {
				if e.entity_id != "" {
					used_ids.insert(e.entity_id.clone());
				}
			});
			
			for (index, entity) in entities.iter_mut().enumerate() {
				println!("[{}] new entity {}", index, entity.template);

				if entity.entity_id == "" {
					loop {
						let new_id = (index + 1).to_string();

						if !used_ids.contains(&new_id) {
							entity.entity_id = new_id;
							break;
						}
					}
				}
	
				tx.send(MapChange::NewMapEntity(entity.clone()));
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

		if let Some(camera_entity) = map.camera {
			println!("new camera entity {:?}", camera_entity);

			tx.send(MapChange::NewCamera(camera_entity));
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
