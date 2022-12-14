#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    None,
    AboutCredits,
    AboutLicense,
    AboutOpenSourceLibs,
    AboutVersion,
    FileLoad,
    FileSave,
    OnlineConnectToIp,
    OnlineCreateServer,
    OnlineProfile,
    OnlineServerList,
    SettingsAudio,
    SettingsControls,
    SettingsDebug,
    SettingsFilesystem,
    SettingsGraphics,
    SettingsLanguage,
    SettingsMods,
    SettingsNetwork,
    SettingsPhysics,
}
