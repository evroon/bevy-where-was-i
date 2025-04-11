use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::{fs::File, io::BufWriter};

use bevy::prelude::*;
use bevy::window::WindowClosing;
use serialization::{deserialize_transform, serialize_transform};

mod serialization;

#[derive(Component)]
#[require(Transform)]
pub struct WhereWasI {
    name: String,
}

impl WhereWasI {
    pub fn from_name(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn camera() -> Self {
        WhereWasI::from_name("camera")
    }
}

#[derive(Resource)]
struct WhereWasIConfig {
    directory: String,
}

pub struct WhereWasIPlugin {
    pub directory: String,
}

impl Default for WhereWasIPlugin {
    fn default() -> Self {
        Self {
            directory: "./assets/saves".into(),
        }
    }
}

impl Plugin for WhereWasIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WhereWasIConfig {
            directory: self.directory.clone(),
        })
        .add_systems(Update, save_state)
        .add_systems(PostStartup, load_state);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_state(mut to_save: Query<(&WhereWasI, &mut Transform)>, config: Res<WhereWasIConfig>) {
    let mut initialized = 0;

    for (where_was_i, mut transform) in to_save.iter_mut() {
        let (directory, filename) = (&config.directory, &where_was_i.name);
        let filepath = format!("{directory}/{filename}.state");

        if let Ok(contents) = read_lines(filepath) {
            match deserialize_transform(contents) {
                Ok(new) => {
                    *transform = new;
                    initialized += 1;
                }
                Err(err) => {
                    error!("Could not deserialize transform: {}", err.message);
                }
            }
        }
    }

    info!("Initialized {} transform(s)", initialized);
}

fn save_state(
    mut events: EventReader<WindowClosing>,
    to_save: Query<(&WhereWasI, &Transform)>,
    config: Res<WhereWasIConfig>,
) {
    let directory = &config.directory;
    let mut saved_files = 0;

    if events.read().next().is_some() {
        for (where_was_i, transform) in to_save.iter() {
            let filename = where_was_i.name.clone();

            if let Ok(false) = fs::exists(directory) {
                fs::create_dir_all(directory).expect("Could not create directory");
            }

            let mut writer = BufWriter::new(
                File::create(format!("{directory}/{filename}.state"))
                    .expect("Error occurred while opening file"),
            );

            #[cfg(not(target_arch = "wasm32"))]
            serialize_transform(&mut writer, transform)
                .expect("Error occurred while writing to disk");

            saved_files += 1;
        }
        info!("Saved {} transforms to: {}", saved_files, directory);
    }
}
