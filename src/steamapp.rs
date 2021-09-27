use std::path::PathBuf;

/// An instance of an installed Steam app.
/// # Example
/// ```rust
/// # use steamlocate::SteamDir;
/// let mut steamdir = SteamDir::locate().unwrap();
/// let gmod = steamdir.app(&4000);
/// println!("{:#?}", gmod.unwrap());
/// ```
/// ```ignore
/// SteamApp (
/// 	appid: u32: 4000,
/// 	path: PathBuf: "C:\\Program Files (x86)\\steamapps\\common\\GarrysMod",
/// 	vdf: <steamy_vdf::Table>,
/// 	name: Some(String: "Garry's Mod"),
/// 	last_user: Some(u64: 76561198040894045) // This will be a steamid_ng::SteamID if the "steamid_ng" feature is enabled
/// )
/// ```
#[derive(Debug, Clone)]
pub struct SteamApp {
	/// The app ID of this Steam app.
	pub appid: u32,

	/// The path to the installation directory of this Steam app.
	///
	/// Example: `C:\Program Files (x86)\Steam\steamapps\common\GarrysMod`
	pub path: PathBuf,
}

impl SteamApp {
	pub(crate) fn new(steamapps: &PathBuf, app_id: u32, installdir: &String) -> Option<SteamApp> {
		// First check if the installation path exists and is a valid directory
		let install_dir = steamapps.join(&installdir);
		if !install_dir.is_dir() { return None }

		Some(SteamApp {
			path: install_dir,
			appid: app_id
		})
	}
}
