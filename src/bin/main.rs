use std::fs;
use std::time::Instant;
use fs_extra::dir::CopyOptions;
use wt_datamine_extractor_lib::custom_loadouts::custom_loadouts::CustomLoadout;
use wt_datamine_extractor_lib::custom_loadouts::known_loadouts::KnownLoadouts;
use wt_datamine_extractor_lib::lang::{ copy_lang};
use wt_datamine_extractor_lib::missile::extract_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::shell::compress::CompressedShells;

use wt_datamine_extractor_lib::shell::known_shells::KnownShells;
use wt_datamine_extractor_lib::shell::shells::{Shell};
use wt_datamine_extractor_lib::thermal::extract_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;


fn main() {
	let start = Instant::now();

	if fs::read_dir("resources/cache").is_ok() {
		fs::write("meta_index/version.txt", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/version").unwrap()).unwrap();
		fs::write("explosive/explosive.blkx", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/gamedata/damage_model/explosive.blkx").unwrap()).unwrap();

		copy_lang();
		copy_loadouts();

		let known_missiles = KnownMissiles::generate_index().write_index().copy_index_to_folder();
		let known_thermals = KnownThermals::generate_index().write_index().copy_index_to_folder();
		let known_shells = KnownShells::generate_index().write_index().copy_index_to_folder();
		let known_loadouts = KnownLoadouts::generate_index().write_index().copy_index_to_folder();

		let missiles = Missile::generate_from_index(&known_missiles);
		let thermals = Thermal::generate_from_index(&known_thermals);
		let shells = Shell::generate_from_index(&known_shells);
		let loadouts = CustomLoadout::generate_from_index(&known_loadouts);


		let compressed_shells = CompressedShells::compress(&shells);
		fs::write("shell_index/compressed.json",serde_json::to_string(&compressed_shells).unwrap()).unwrap();

		Missile::write_all(missiles);
		Thermal::write_all(thermals);
		Shell::write_all(shells);
		CustomLoadout::write_all(loadouts);
	} else {
		panic!("Local mined cache is invalid or could not be read");
	}

	println!("{:?}", start.elapsed());
}

pub fn copy_loadouts() {
	let options = CopyOptions {
		overwrite: true,
		skip_exist: false,
		buffer_size: 10_000,
		copy_inside: true,
		content_only: false,
		depth: 0,
	};

	fs_extra::dir::copy("resources/cache/aces.vromfs.bin_u/gamedata/weapons", "./custom_loadouts", &options).unwrap();
}