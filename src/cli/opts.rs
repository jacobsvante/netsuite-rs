use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

use super::env::EnvVar;
use crate::metadata::VERSION;

#[derive(Debug, Parser)]
#[clap(name = "netsuite", version = VERSION)]
pub(crate) struct Opts {
    #[clap(short = 's', long, env, default_value = "netsuite")]
    ini_section: String,
    /// Where to load INI from, defaults to your OS's config directory.
    #[clap(short = 'p', long, env)]
    ini_path: Option<PathBuf>,
    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
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
pub(crate) enum SubCommand {
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
