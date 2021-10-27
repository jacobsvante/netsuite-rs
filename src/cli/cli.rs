use std::{
    io::{stdout, Write},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use crate::config::Config;
use crate::error::Error;
use crate::rest_api::RestApi;
use clap::Parser;
use log::{LevelFilter, debug};
use super::ini;
use super::env::EnvVar;

#[derive(Debug, Parser)]
#[clap(name = "netsuite", version = "abc123")]
pub(crate) struct Opts {
    #[clap(short = 's', long, env, default_value = "netsuite")]
    ini_section: String,
    /// Where to load INI from, defaults to your OS's config directory.
    #[clap(short = 'p', long, env)]
    ini_path: Option<PathBuf>,
    #[clap(subcommand)]
    subcmd: SubCommand,
    /// Set the log level
    #[clap(
        short = 'l',
        long = "log-level",
        value_name = "level",
        env = "LOG_LEVEL"
    )]
    level_filter: Option<LevelFilter>,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[clap(name = "suiteql")]
    SuiteQl {
        /// The query to execute. If `-` is provided, query will be read from standard input.
        query: String,
        #[clap(short, long, env = EnvVar::Account.into())]
        account: String,
        #[clap(short = 'c', long, env = EnvVar::ConsumerKey.into())]
        consumer_key: String,
        #[clap(short = 'C', long, env = EnvVar::ConsumerSecret.into())]
        consumer_secret: String,
        #[clap(short = 't', long, env = EnvVar::TokenId.into())]
        token_id: String,
        #[clap(short = 'T', long, env = EnvVar::TokenSecret.into())]
        token_secret: String,
        #[clap(short, long, default_value = "1000")]
        limit: usize,
        #[clap(short, long, default_value = "0")]
        offset: usize,
    },
    #[clap(name = "default-ini-path")]
    DefaultIniPath,
}

pub fn run() -> Result<(), Error> {

    if let Err(err) = ini::to_env() {
        debug!("Couldn't load INI: {}", err);
    };

    let cli_opts = Opts::parse();

    if let Some(level_filter) = cli_opts.level_filter {
        env_logger::builder().filter(None, level_filter).init();
    }

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
                &account,
                &consumer_key,
                &consumer_secret,
                &token_id,
                &token_secret,
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

