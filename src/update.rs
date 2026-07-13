use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;
use std::time::Duration;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_NAME: &str = "lazarobox-img";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    NotChecked,
    Checking,
    UpToDate,
    UpdateAvailable,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateCheck {
    pub latest_version: String,
    pub status: UpdateStatus,
}

#[derive(Debug, Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    package: CratesIoPackage,
}

#[derive(Debug, Deserialize)]
struct CratesIoPackage {
    max_stable_version: Option<String>,
    newest_version: String,
}

/// Comprueba la última versión estable publicada en crates.io.
pub fn check_latest_stable() -> Result<UpdateCheck> {
    let latest_version = fetch_latest_stable_version()?;
    let status = compare_versions(CURRENT_VERSION, &latest_version)?;

    Ok(UpdateCheck {
        latest_version,
        status,
    })
}

/// Descarga la última versión estable del paquete desde la API de crates.io.
pub fn fetch_latest_stable_version() -> Result<String> {
    let url = format!("https://crates.io/api/v1/crates/{CRATE_NAME}");
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build()
        .into();

    let body = agent
        .get(&url)
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            concat!("lazarobox-img/", env!("CARGO_PKG_VERSION")),
        )
        .call()
        .context("no se pudo contactar con crates.io")?
        .body_mut()
        .read_to_string()
        .context("no se pudo leer la respuesta de crates.io")?;

    latest_stable_version_from_json(&body)
}

fn latest_stable_version_from_json(body: &str) -> Result<String> {
    let response: CratesIoResponse =
        serde_json::from_str(body).context("respuesta inválida de crates.io")?;

    let version = response
        .package
        .max_stable_version
        .filter(|version| !version.trim().is_empty())
        .or_else(|| stable_newest_version(response.package.newest_version));

    version.context("crates.io no devolvió una versión estable")
}

fn stable_newest_version(version: String) -> Option<String> {
    let parsed = Version::parse(version.trim()).ok()?;

    if parsed.pre.is_empty() {
        Some(version)
    } else {
        None
    }
}

pub fn compare_versions(current: &str, latest: &str) -> Result<UpdateStatus> {
    let current = Version::parse(current).context("versión actual inválida")?;
    let latest = Version::parse(latest).context("última versión inválida")?;

    if latest > current {
        Ok(UpdateStatus::UpdateAvailable)
    } else {
        Ok(UpdateStatus::UpToDate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_versions_detects_available_update() {
        let status = compare_versions("0.4.2", "0.5.0").unwrap();

        assert_eq!(status, UpdateStatus::UpdateAvailable);
    }

    #[test]
    fn compare_versions_treats_equal_or_older_latest_as_up_to_date() {
        assert_eq!(
            compare_versions("0.4.2", "0.4.2").unwrap(),
            UpdateStatus::UpToDate
        );
        assert_eq!(
            compare_versions("0.4.2", "0.4.1").unwrap(),
            UpdateStatus::UpToDate
        );
    }

    #[test]
    fn latest_stable_version_prefers_max_stable_version() {
        let body = r#"{"crate":{"max_stable_version":"0.4.2","newest_version":"0.5.0-alpha.1"}}"#;

        assert_eq!(latest_stable_version_from_json(body).unwrap(), "0.4.2");
    }

    #[test]
    fn latest_stable_version_falls_back_to_newest_version() {
        let body = r#"{"crate":{"max_stable_version":null,"newest_version":"0.4.2"}}"#;

        assert_eq!(latest_stable_version_from_json(body).unwrap(), "0.4.2");
    }

    #[test]
    fn latest_stable_version_rejects_prerelease_newest_fallback() {
        let body = r#"{"crate":{"max_stable_version":null,"newest_version":"0.5.0-alpha.1"}}"#;

        let error = latest_stable_version_from_json(body).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("crates.io no devolvió una versión estable")
        );
    }
}
