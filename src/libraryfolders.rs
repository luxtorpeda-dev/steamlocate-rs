use std::path::PathBuf;
use keyvalues_parser::Vdf;

/// An instance which contains all the Steam library folders installed on the file system.
/// Example:
/// ```rust
/// # use std::{vec, path::PathBuf};
/// # use steamlocate::{SteamDir, LibraryFolders};
/// let mut steamdir: SteamDir = SteamDir::locate().unwrap();
/// let libraryfolders: &LibraryFolders = steamdir.libraryfolders();
/// let paths: &Vec<PathBuf> = &libraryfolders.paths;
/// println!("{:#?}", paths);
/// ```
/// ```ignore
/// {
///		"C:\\Program Files (x86)\\Steam\\steamapps",
///		"D:\\Steam\\steamapps",
///		"E:\\Steam\\steamapps",
///		"F:\\Steam\\steamapps",
///		...
///	}
/// ```
#[derive(Default, Clone, Debug)]
pub struct LibraryFolders {
	/// A `Vec<PathBuf>` of Steam library folder paths.
	///
	/// This will always include the Steam installation directory's `SteamApps` folder.
	pub paths: Vec<PathBuf>,
	pub(crate) discovered: bool
}

impl LibraryFolders {
	pub(crate) fn discover(&mut self, path: &PathBuf) {
		let mut steamapps_name = "SteamApps";
		
		let mut steamapps = path.join(&steamapps_name);
		
		if !steamapps.is_dir() {
			steamapps_name = "steamapps";
			steamapps = path.join(&steamapps_name);
		}

		// from https://github.com/LovecraftianHorror/vdf-rs/issues/25
		let libraryfolders_vdf_path = steamapps.join("libraryfolders.vdf");
		if libraryfolders_vdf_path.is_file() {
			let vdf_text = match std::fs::read_to_string(libraryfolders_vdf_path) {
				Ok(s) => s,
				Err(_err) => {
					println!("discover. vdf read error");
					return;
				}
			};
			
			let Vdf { key, value } = match Vdf::parse(&vdf_text) {
				Ok(s) => s,
				Err(_err) => {
					println!("discover. vdf parse error");
					return
				}
			};
			
			if key != "libraryfolders" {
				println!("discover. key incorrect {}", key);
				return
			}
			
			let mut obj = value.unwrap_obj();
			let mut index = 0;
			while !obj.is_empty() {
				match obj.remove_entry(index.to_string().as_str()) {
					Some(item) => {
						let key = item.0;
						let mut values = item.1;
						
						let value = values.pop().unwrap();
						let library_info_vdf = Vdf { key, value };
						let info_obj = library_info_vdf.value.unwrap_obj();
						
						let path_arr = &mut info_obj.get("path").unwrap();
						let path_obj = path_arr.clone().pop().unwrap();
						match path_obj.get_str() {
							Some(path) => {
								let mut new_path = PathBuf::from(path);
								if new_path.exists() {
									new_path = new_path.join(steamapps_name);
									println!("found library path at {:#?}", new_path);
									self.paths.push(new_path);
								} else {
									println!("found library path at {:#?} is not accessible", path);
								}
							},
							None => {}
						}
					},
					None => {
						break;
					}
				}

				index += 1;
			}
		}
		
		self.discovered = true;
	}
}
