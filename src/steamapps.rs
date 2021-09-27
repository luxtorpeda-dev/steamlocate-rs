use regex::Regex;
use crate::steamapp::SteamApp;
use crate::libraryfolders::LibraryFolders;
use std::collections::HashMap;
use keyvalues_parser::Vdf;

lazy_static! {
	static ref APPMANIFEST_RE: Regex = Regex::new(r"^appmanifest_(\d+)\.acf$").unwrap();
}

#[derive(Default, Clone, Debug)]
pub(crate) struct SteamApps {
	pub(crate) apps: HashMap<u32, Option<SteamApp>>,
	pub(crate) discovered: bool
}

impl SteamApps {
	pub(crate) fn discover_app(&mut self, libraryfolders: &LibraryFolders, app_id: &u32) -> Option<()> {
		for libraryfolder in &libraryfolders.paths {
			let mut appmanifest_path = libraryfolder.join(format!("appmanifest_{}.acf", app_id));
			if appmanifest_path.is_file() {
				let vdf_text = match std::fs::read_to_string(&appmanifest_path) {
					Ok(s) => s,
					Err(_err) => {
						println!("discover_app. vdf read error");
						return None;
					}
				};
				
				let Vdf { key, value } = match Vdf::parse(&vdf_text) {
					Ok(s) => s,
					Err(_err) => {
						println!("discover_app. vdf parse error");
						return None;
					}
				};
				
				if key != "AppState" {
					println!("discover_app. key incorrect {}", key);
					return None;
				}
				
				appmanifest_path.pop(); appmanifest_path.push("common");
				
				let info_obj = value.unwrap_obj();
				let mut installdir = String::from("");
				
				let installdir_arr = &mut info_obj.get("installdir").unwrap();
				let installdir_obj = installdir_arr.clone().pop().unwrap();
				match installdir_obj.get_str() {
					Some(installdir_str) => {
						installdir = installdir_str.to_string();
					},
					None => {}
				}

				self.apps.insert(
					*app_id,
					SteamApp::new(&appmanifest_path, *app_id, &installdir)
				);

				return Some(())
			}
		}

		None
	}
}
