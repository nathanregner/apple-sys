// TODO: change this module to thin wrapper of apple_sdk::SdkPath and related utilities

use apple_sdk::{AppleSdk, Platform, SimpleSdk};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SdkPathError {
    #[error("apple_sdk::Error")]
    AppleSdkError(apple_sdk::Error),
    #[error("sdk not found")]
    AppleSdkNotFound,
    #[error("path is not sdk path")]
    InvalidPath(PathBuf),
    #[error("xcrun lookup failed")]
    XcrunError(std::io::Error),
}

#[derive(Debug, Clone)]
pub struct SdkPath(PathBuf);

impl SdkPath {
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl TryFrom<&Platform> for SdkPath {
    type Error = SdkPathError;

    fn try_from(platform: &Platform) -> Result<Self, Self::Error> {
        let sdks = apple_sdk::SdkSearch::default()
            .platform(platform.clone())
            .search::<SimpleSdk>()
            .map_err(SdkPathError::AppleSdkError)?;
        let sdk = sdks
            .into_iter()
            .next()
            .ok_or(SdkPathError::AppleSdkNotFound)?;
        let path = sdk.path();
        Ok(Self(PathBuf::from(path)))
    }
}

impl TryFrom<PathBuf> for SdkPath {
    type Error = SdkPathError;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let s = path
            .to_str()
            .expect("sdk path is always convertible to utf-8 string");
        if !s.ends_with(".sdk") {
            return Err(SdkPathError::InvalidPath(path));
        }
        let path = std::path::PathBuf::from(s);
        if !path.exists() {
            return Err(SdkPathError::InvalidPath(path));
        }
        Ok(Self(path))
    }
}

impl TryFrom<&str> for SdkPath {
    type Error = SdkPathError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let platform = Platform::from_str(s).map_err(SdkPathError::AppleSdkError)?;
        SdkPath::try_from(&platform)
    }
}
