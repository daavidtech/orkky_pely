use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use bevy::prelude::Resource;
use notify::RecursiveMode;
use notify::Watcher;

use crate::map::Map;
use crate::map::MapChange;

fn emit_changes(
	last_map: &Map, 
	new_map: &mut Map,
	tx: &mpsc::Sender<MapChange>
) {
	match new_map.entities {
		Some(ref mut entities) => {
			let mut used_ids = HashSet::new();

			entities.iter().for_each(|e| {
				if e.entity_id != "" {
					used_ids.insert(e.entity_id.clone());
				}
			});
			
			for (index, entity) in entities.iter_mut().enumerate() {
				if entity.entity_id == "" {
					loop {
						let new_id = (index + 1).to_string();

						if !used_ids.contains(&new_id) {
							entity.entity_id = new_id;
							break;
						}
					}
				}

				let last_map_entity = match &last_map.entities {
					Some(last_entities) => {
						last_entities.iter().find(|e| e.entity_id == entity.entity_id)
					},
					None => None
				};

				match last_map_entity {
					Some(last_entity) => {
						if last_entity != entity {
							println!("emit entity update ({:?} -> {:?})", last_entity, entity);

							tx.send(MapChange::UpdateMapEntity(entity.clone())).unwrap();
						}
					},
					None => {
						println!("emit new entity {:?}", entity);

						tx.send(MapChange::NewMapEntity(entity.clone())).unwrap();
					}
				}
			}
		},
		None => {
			for entity in last_map.entities.as_ref().unwrap() {
				tx.send(MapChange::RemoveMapEntity(entity.entity_id.clone())).unwrap();
			}
		}
	}

	match new_map.templates {
		Some(ref mut templates) => {
			for template in templates {
				let last_map_template = match &last_map.templates {
					Some(last_templates) => {
						last_templates.iter().find(|t| t.name == template.name)
					},
					None => None
				};

				match last_map_template {
					Some(last_template) => {
						if last_template != template {
							tx.send(MapChange::UpdateMaptemplate(template.clone())).unwrap();
						}
					},
					None => {
						tx.send(MapChange::NewMapTemplate(template.clone())).unwrap();
					}
				}
			}
		},
		None => {
			for template in last_map.templates.as_ref().unwrap() {
				tx.send(MapChange::RemoveMapTemplate(template.name.clone())).unwrap();
			}
		}
	}

	match new_map.shapes {
		Some(ref mut shapes) => {
			let mut used_ids = HashSet::new();

			shapes.iter().for_each(|s| {
				if s.id != "" {
					used_ids.insert(s.id.clone());
				}
			});

			for (index, shape) in shapes.iter_mut().enumerate() {
				if shape.id == "" {
					loop {
						let new_id = (index + 1).to_string();

						if !used_ids.contains(&new_id) {
							shape.id = new_id;
							break;
						}
					}
				}

				let existing_shape = match &last_map.shapes {
					Some(last_shapes) => {
						last_shapes.iter().find(|s| s.id == shape.id)
					},
					None => None
				};

				match existing_shape {
					Some(existing_shape) => {
						if existing_shape != shape {
							tx.send(MapChange::UpdateMapShape(shape.clone())).unwrap();
						}
					},
					None => {
						tx.send(MapChange::NewMapShape(shape.clone())).unwrap();
					}
				}
			}
		},
		None => {
			for shape in last_map.shapes.as_ref().unwrap() {
				tx.send(MapChange::RemoveMapShape(shape.id.clone())).unwrap();
			}
		}
	}

	match new_map.lights {
		Some(ref mut lights) => {
			let mut used_ids = HashSet::new();

			lights.iter().for_each(|l| {
				if l.id != "" {
					used_ids.insert(l.id.clone());
				}
			});

			for (index, light) in lights.iter_mut().enumerate() {
				if light.id == "" {
					loop {
						let new_id = (index + 1).to_string();

						if !used_ids.contains(&new_id) {
							light.id = new_id;
							break;
						}
					}
				}

				let existing_light = match &last_map.lights {
					Some(last_lights) => {
						last_lights.iter().find(|l| l.id == light.id)
					},
					None => None
				};

				match existing_light {
					Some(existing_light) => {
						if existing_light != light {
							tx.send(MapChange::UpdateLight(light.clone())).unwrap();
						}
					},
					None => {
						tx.send(MapChange::NewLight(light.clone())).unwrap();
					}
				}
			}
		},
		None => {
			for light in last_map.lights.as_ref().unwrap() {
				tx.send(MapChange::RemoveLight(light.id.clone())).unwrap();
			}
		}
	};


	match new_map.ambient_light {
		Some(ref mut light) => {
			match &last_map.ambient_light {
				Some(last_ambient_light) => {
					if last_ambient_light != light {
						tx.send(MapChange::UpdateAmbientLight(light.clone()));
					}
				},
				None => {
					tx.send(MapChange::NewAmbientLight(light.clone()));
				}
			}
		},
		None =>  {
			tx.send(MapChange::RemoveAmbientLight);
		},
	}


	match new_map.camera {
		Some(ref mut camera) => {
			match &last_map.camera {
				Some(last_camera) => {
					if last_camera != camera {
						tx.send(MapChange::UpdateCamera(camera.clone()));
					}
				},
				None => {
					tx.send(MapChange::NewCamera(camera.clone()));
				}
			}
		},
		None =>  {
			tx.send(MapChange::RemoveCamera);
		},
	}
}

pub fn create_map_loader(path: &str) -> MapChangesReceiver {
	let (tx, rx) = std::sync::mpsc::channel();

	let path = path.to_string();

	thread::spawn(move || {
		let tx = tx;

		let mut last_map = Map::default();

		let path = path;

		match Map::load(&path) {
			Ok(mut new_map) => {
				emit_changes(&last_map, &mut new_map, &tx);

				last_map = new_map;
			},
			Err(err) => {
				println!("error loading map: {}", err);
			}
		};

		println!("starting map loader worker {}", path);

		let (fs_change_tx, fs_change_rx) = std::sync::mpsc::channel();
		let mut watcher = notify::recommended_watcher(fs_change_tx).unwrap();

		println!("adding path to watcher: {}", path);

		// Add a path to be watched. All files and directories at that path and
		// below will be monitored for changes.
		watcher.watch(Path::new(&path), RecursiveMode::Recursive).unwrap();

		for fs_change in fs_change_rx {
			println!("fs change: {:?}", fs_change);

			match Map::load(&path) {
				Ok(mut new_map) => {
					emit_changes(&last_map, &mut new_map, &tx);

					last_map = new_map;
				},
				Err(err) => {
					println!("error loading map: {}", err);
				}
			};
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
