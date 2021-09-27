use std::path::PathBuf;
use keyvalues_parser::Vdf;
use keyvalues_serde::{from_str, Error as KeyValuesSerdeError, Result as KeyValuesSerdeResult};
use serde::Deserialize;

use std::str::FromStr;

// from https://github.com/LovecraftianHorror/vdf-rs/issues/25
#[derive(Debug, Clone)]
struct RawLibraryFolders {
    libraries: Vec<LibraryInfo>
}

impl FromStr for RawLibraryFolders {
    type Err = KeyValuesSerdeError;

    fn from_str(s: &str) -> KeyValuesSerdeResult<Self> {
        let Vdf { key, value } = Vdf::parse(s)?;
        
        let mut obj = value.unwrap_obj();

        let mut libraries = Vec::with_capacity(obj.len());
        let mut index = 0;
        while !obj.is_empty() {
			println!("{}", index);
            let (key, mut values) = obj.remove_entry(index.to_string().as_str()).unwrap();
            println!("{} 2", index);

            let value = values.pop().unwrap();
            let library_info_vdf = Vdf { key, value };
            let info_obj = library_info_vdf.value.unwrap_obj();
            
            let path = info_obj.get("path");
            
            //let library_info: LibraryInfo = from_str(&library_info_vdf.to_string())?;

            //libraries.push(library_info);
            
            println!("{:#?}", path);

            index += 1;
        }

        Ok(Self {
            libraries,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct LibraryInfo {
    path: String
}

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
		
		self.paths.push(steamapps.clone());

		let libraryfolders_vdf_path = steamapps.join("libraryfolders.vdf");
		
		if libraryfolders_vdf_path.is_file() {
			let vdf_text = match std::fs::read_to_string(libraryfolders_vdf_path) {
				Ok(s) => s,
				Err(_err) => {
					 return;
				}
			};
			
			let library_folders: RawLibraryFolders = vdf_text.parse().unwrap();
			println!("{:#?}", library_folders);
			
			
		}
		
		self.discovered = true;
	}
}
