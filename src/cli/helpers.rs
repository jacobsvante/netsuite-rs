use std::{fmt::Display, str::FromStr};

use clap::IntoApp;

use super::opts::Opts;

pub(crate) fn safe_extract_arg<T>(arg_id: &str) -> Option<T>
where
    T: FromStr,
    T::Err: Display,
{
    let app = Opts::command().ignore_errors(true);
    app.get_matches().value_of_t::<T>(arg_id).ok()
}
