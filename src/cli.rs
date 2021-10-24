use crate::config::Config;
use crate::error::Error;
use crate::rest_api::RestApi;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "netsuite", version = "abc123")]
struct Opts {
    #[clap(short, long, env)]
    account_id: String,
    #[clap(short = 'c', long, env)]
    consumer_key: String,
    #[clap(short = 'C', long, env)]
    consumer_secret: String,
    #[clap(short = 't', long, env)]
    token_key: String,
    #[clap(short = 'T', long, env)]
    token_secret: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "suiteql")]
    SuiteQl {
        /// The query to execute. If `-` is provided, query will be read from standard input.
        query: String,
        #[clap(short, long, default_value = "1000")]
        limit: usize,
        #[clap(short, long, default_value = "0")]
        offset: usize,
    },
}

pub fn run() -> Result<(), Error> {
    env_logger::init();

    let opts = Opts::parse();
    let config = Config::new(
        &opts.account_id,
        &opts.consumer_key,
        &opts.consumer_secret,
        &opts.token_key,
        &opts.token_secret,
    );
    let api = RestApi::new(&config);

    match &opts.subcmd {
        SubCommand::SuiteQl { query, limit, offset } => {
            let result = api.suiteql.raw(query, *limit, *offset)?;
            println!("{}", result);
        }
    }

    Ok(())
}
