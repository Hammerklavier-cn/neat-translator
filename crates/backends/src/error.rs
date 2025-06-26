use std::path::PathBuf;

// #[derive(Debug)]
// struct ConfigDirIsFileError {
//     path: String,
// }

// impl ConfigDirIsFileError {
//     fn new(path: &str) -> Self {
//         Self {
//             path: path.to_string(),
//         }
//     }
// }

// impl std::fmt::Display for ConfigDirIsFileError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Config directory '{}' is a file", self.path)
//     }
// }

// impl std::error::Error for ConfigDirIsFileError {}

#[derive(Debug)]
pub enum Error {
    ConfigDirIsFile(PathBuf),
    ConfigFileIsDir(PathBuf),
    ConfigFileBadFormat(PathBuf, String),
}
impl Error {
    pub(super) fn new_config_dir_is_file(path: PathBuf) -> Self {
        Self::ConfigDirIsFile(path)
    }

    pub(super) fn new_config_file_is_dir(path: PathBuf) -> Self {
        Self::ConfigFileIsDir(path)
    }

    pub(super) fn new_config_file_bad_format(path: PathBuf, context: String) -> Self {
        Self::ConfigFileBadFormat(path, context)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ConfigDirIsFile(path) => {
                write!(f, "Config directory '{}' is a file", path.display())
            }
            Error::ConfigFileIsDir(path) => {
                write!(f, "Config file '{}' is a directory", path.display())
            }
            Error::ConfigFileBadFormat(path, context) => {
                write!(
                    f,
                    "Config file '{}' has a bad format: {}",
                    path.display(),
                    context
                )
            }
        }
    }
}
impl std::error::Error for Error {}
