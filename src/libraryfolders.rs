use std::path::PathBuf;
use keyvalues_parser::Vdf;
use std::io;
use std::io::{Error, ErrorKind};

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
	pub(crate) fn discover(&mut self, path: &PathBuf) -> io::Result<()> {
		let mut steamapps_name = "SteamApps";
		
		let mut steamapps = path.join(steamapps_name);
		
		if !steamapps.is_dir() {
			steamapps_name = "steamapps";
			steamapps = path.join(steamapps_name);
		}
		
		self.paths.push(steamapps.clone());

		let libraryfolders_vdf_path = steamapps.join("libraryfolders.vdf");
		
		if libraryfolders_vdf_path.is_file() {
			let vdf_text = match std::fs::read_to_string(libraryfolders_vdf_path) {
				Ok(s) => s,
				Err(err) => {
					 return Err(Error::new(ErrorKind::Other, "choices array? null"));
				}
			};
			let vdf = match Vdf::parse(&vdf_text) {
				Ok(s) => s,
				Err(err) => {
					 return Err(Error::new(ErrorKind::Other, "choices array null"));
				}
			};
			println!("vdf. {:#?}.", vdf);
		}
		
		self.discovered = true;
		Ok(())
	}
}
