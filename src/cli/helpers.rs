use std::{fmt::Display, str::FromStr};

use clap::{AppSettings, IntoApp};

use super::cli::Opts;

pub(crate) fn safe_extract_arg<T>(arg_id: &str) -> Option<T>
where
    T: FromStr,
    T::Err: Display,
{
    let app = Opts::into_app().global_setting(AppSettings::IgnoreErrors);
    app.get_matches().value_of_t::<T>(arg_id).ok()
}
