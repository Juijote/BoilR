use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpicGamesLauncherSettings {
    pub enabled: bool,
    pub location: Option<String>,

    #[cfg(target_os = "linux")]
    pub create_symlinks: bool,
}
