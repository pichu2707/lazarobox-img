use anyhow::{Context, Result, bail};
use semver::Version;
use serde::Deserialize;
use std::process::{Command, Output};
use std::time::Duration;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_NAME: &str = "lazarobox-img";
pub const HOMEBREW_FORMULA: &str = "pichu2707/tap/lazarobox-img";
pub const HOMEBREW_INSTALL_COMMAND: &str = "brew install pichu2707/tap/lazarobox-img";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    NotChecked,
    Checking,
    UpToDate,
    UpdateAvailable,
    Confirming,
    Updating,
    Updated,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateCheck {
    pub latest_version: String,
    pub status: UpdateStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrewCommand {
    pub program: &'static str,
    pub args: Vec<&'static str>,
}

impl BrewCommand {
    pub fn display(&self) -> String {
        std::iter::once(self.program)
            .chain(self.args.iter().copied())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomebrewUpdateResult {
    pub formula: String,
    pub output: String,
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

pub fn homebrew_update_commands() -> Vec<BrewCommand> {
    vec![
        BrewCommand {
            program: "brew",
            args: vec!["update"],
        },
        BrewCommand {
            program: "brew",
            args: vec!["upgrade", HOMEBREW_FORMULA],
        },
    ]
}

pub fn homebrew_update_command_text() -> String {
    homebrew_update_commands()
        .iter()
        .map(BrewCommand::display)
        .collect::<Vec<_>>()
        .join(" && ")
}

pub fn run_homebrew_update() -> Result<HomebrewUpdateResult> {
    ensure_homebrew_available()?;
    let formula = installed_homebrew_formula()?;
    let commands = homebrew_update_commands();
    let mut log = String::new();

    for command in commands {
        let output = run_brew_command(&command.args)?;
        append_command_output(&mut log, &command, &output);

        if !output.status.success() {
            bail!(
                "Homebrew devolvió un error al ejecutar `{}`.\n{}",
                command.display(),
                format_output(&output)
            );
        }
    }

    Ok(HomebrewUpdateResult {
        formula,
        output: log,
    })
}

fn ensure_homebrew_available() -> Result<()> {
    let output = Command::new("brew")
        .arg("--version")
        .output()
        .context(homebrew_install_message())?;

    if output.status.success() {
        Ok(())
    } else {
        bail!(homebrew_install_message())
    }
}

fn installed_homebrew_formula() -> Result<String> {
    for formula in [HOMEBREW_FORMULA, CRATE_NAME] {
        let output = run_brew_command(&["list", "--versions", formula])?;
        if output.status.success() && !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
            return Ok(formula.to_string());
        }
    }

    bail!(homebrew_install_message())
}

fn run_brew_command(args: &[&str]) -> Result<Output> {
    Command::new("brew")
        .args(args)
        .output()
        .context(homebrew_install_message())
}

fn homebrew_install_message() -> String {
    format!(
        "Actualización desde la TUI disponible solo con Homebrew. Instala la fórmula con `{HOMEBREW_INSTALL_COMMAND}` para activarla."
    )
}

fn append_command_output(log: &mut String, command: &BrewCommand, output: &Output) {
    if !log.is_empty() {
        log.push('\n');
    }
    log.push_str("$ ");
    log.push_str(&command.display());
    log.push('\n');
    log.push_str(&format_output(output));
}

fn format_output(output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut text = String::new();

    text.push_str("resultado: ");
    text.push_str(&output.status.to_string());
    text.push('\n');

    if !stdout.trim().is_empty() {
        text.push_str("stdout:\n");
        text.push_str(stdout.trim());
        text.push('\n');
    }
    if !stderr.trim().is_empty() {
        text.push_str("stderr:\n");
        text.push_str(stderr.trim());
        text.push('\n');
    }
    if stdout.trim().is_empty() && stderr.trim().is_empty() {
        text.push_str("sin salida\n");
    }

    text
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

    #[test]
    fn homebrew_update_commands_use_tapped_formula_without_shell() {
        let commands = homebrew_update_commands();

        assert_eq!(
            commands,
            vec![
                BrewCommand {
                    program: "brew",
                    args: vec!["update"]
                },
                BrewCommand {
                    program: "brew",
                    args: vec!["upgrade", "pichu2707/tap/lazarobox-img"]
                }
            ]
        );
        assert_eq!(
            homebrew_update_command_text(),
            "brew update && brew upgrade pichu2707/tap/lazarobox-img"
        );
    }
}
