use std::{
    io::{stdout, Write},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;
use log::{debug, LevelFilter};

use super::opts::{Opts, SubCommand};
use super::{helpers::safe_extract_arg, ini};
use crate::config::Config;
use crate::error::Error;
use crate::rest_api::RestApi;

pub fn run() -> Result<(), Error> {
    if let Some(level_filter) = safe_extract_arg::<LevelFilter>("level-filter") {
        env_logger::builder().filter(None, level_filter).init();
    }

    if let Err(err) = ini::to_env() {
        debug!("Couldn't load INI: {}", err);
    };

    let cli_opts = Opts::parse();

    match &cli_opts.subcmd {
        SubCommand::SuiteQl {
            query,
            account,
            consumer_key,
            consumer_secret,
            token_id,
            token_secret,
            limit,
            offset,
        } => {
            let config = Config::new(
                account,
                consumer_key,
                consumer_secret,
                token_id,
                token_secret,
            );
            let api = RestApi::new(&config);
            let result = api.suiteql.raw(query, *limit, *offset)?;
            println!("{}", result);
        }
        SubCommand::DefaultIniPath => {
            ini::default_location().map(|p| stdout().write(p.as_os_str().as_bytes()));
        }
    }

    Ok(())
}
