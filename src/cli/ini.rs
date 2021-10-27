use super::cli::Opts;
use super::env::EnvVar;
use super::CliError;
use clap::{AppSettings, IntoApp};
use configparser::ini::Ini;
use log::debug;
use std::path::PathBuf;

pub(crate) fn default_location() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("netsuite.ini"))
}

/// Ensure that configured ini values are exported as environment variables, so
/// that they can later be loaded by Opts.
// TODO: This is not very pretty. Submit issue/PR to clap for INI support.
pub(crate) fn to_env() -> Result<(), CliError> {
    let maybe_ini_path = &default_location().map(|p| p.into_os_string());
    let (ini_path, ini_section) = {
        let app = Opts::into_app()
            .global_setting(AppSettings::IgnoreErrors)
            .mut_arg("ini-path", |arg| match maybe_ini_path {
                Some(p) => arg.default_value_os(p),
                None => arg,
            });
        let matches = app.get_matches();
        let ini_section: String = matches.value_of_t_or_exit("ini-section");
        let ini_path: PathBuf = match matches.value_of_t("ini-path") {
            Ok(p) => p,
            Err(_) => return Err(CliError::MissingIniPath),
        };
        (ini_path, ini_section)
    };

    let mut ini = Ini::new();
    if ini_path.exists() {
        debug!("Loaded INI {:?}", &ini_path);
        ini.load(&ini_path).unwrap_or_default();
    } else {
        debug!("INI {:?} doesn't exist. Nothing loaded.", &ini_path);
    }

    let section = ini.remove_section(&ini_section);
    if section.is_some() {
        debug!("Loaded INI section {}.", &ini_section);
    } else {
        debug!("No such INI section: {}.", &ini_section);
    }

    for (k, v) in section.unwrap_or_default() {
        let k = k.to_ascii_uppercase();
        if let Some(v) = v {
            if let Err(err) = EnvVar::set(&k, &v) {
                debug!("{}", err);
            }
        }
    }

    Ok(())
}
