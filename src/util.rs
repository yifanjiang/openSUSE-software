use std::fmt;

pub enum PKmessage {
    PackageListNew(Vec<PackageInfo>),
    PackageListInstalled(Vec<PackageInfo>),
    Progress((i32, Option<String>)),
    //SearchListNew(Vec<SearchInfo>),
    DownloadFinish,
    UpdateFinish,
    InstallFinish,
    RemoveFinish,
    Error(String),
}

#[derive(Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version_current: String,
    pub version_new: String,
}

#[derive(Clone)]
pub struct SearchInfo {
    pub name: String,
    pub id: String,
    pub summary: String,
    pub info: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ButtonState {
    Refresh,
    Refreshing,
    Download,
    Downloading,
    Update,
    Updating,
    RestartUpdate,
}

impl fmt::Display for ButtonState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Refresh => write!(f, "{}", "Refresh"),
            Self::Refreshing => write!(f, "{}", "Refreshing"),
            Self::Download => write!(f, "{}", "Download"),
            Self::Downloading => write!(f, "{}", "Downloading"),
            Self::Update => write!(f, "{}", "Update"),
            Self::Updating => write!(f, "{}", "Updating"),
            Self::RestartUpdate => write!(f, "{}", "Restart&Update"),
        }
    }
}
