use std::{
    fs,
    io::{stdout, Write},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;
use log::{debug, LevelFilter};

use super::opts::{Opts, RestApiSubCommand, SubCommand};
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

    match cli_opts.subcmd {
        SubCommand::RestApiOpts {
            account,
            consumer_key,
            consumer_secret,
            token_id,
            token_secret,
            subcmd,
        } => {
            let config = Config::new(
                account,
                consumer_key,
                consumer_secret,
                token_id,
                token_secret,
            );
            let api = RestApi::new(config);
            rest_api_sub_command(subcmd, api)?
        }
        SubCommand::DefaultIniPath => {
            ini::default_location().map(|p| stdout().write(p.as_os_str().as_bytes()));
        }
    }

    Ok(())
}

fn rest_api_sub_command(subcmd: RestApiSubCommand, api: RestApi) -> Result<(), Error> {
    use RestApiSubCommand::*;
    let response = match subcmd {
        RestApiSubCommand::SuiteQl {
            query,
            limit,
            offset,
        } => api.suiteql.raw(&query, limit, offset)?,
        Get {
            endpoint,
            params,
            headers,
        } => {
            let params = if params.is_empty() {
                None
            } else {
                Some(params.into())
            };
            let headers = if headers.is_empty() {
                None
            } else {
                Some(headers.into())
            };
            api.get_raw(&endpoint, params, headers)?
        }
        Post {
            endpoint,
            file,
            params,
            headers,
        } => {
            let params = if params.is_empty() {
                None
            } else {
                Some(params.into())
            };
            let headers = if headers.is_empty() {
                None
            } else {
                Some(headers.into())
            };
            let payload = if let Some(file) = file {
                Some(fs::read_to_string(file)?)
            } else {
                None
            };
            api.post_raw(&endpoint, params, headers, payload.as_deref())?
        }
        Put {
            endpoint,
            file,
            params,
            headers,
        } => {
            let params = if params.is_empty() {
                None
            } else {
                Some(params.into())
            };
            let headers = if headers.is_empty() {
                None
            } else {
                Some(headers.into())
            };
            let payload = if let Some(file) = file {
                Some(fs::read_to_string(file)?)
            } else {
                None
            };
            api.put_raw(&endpoint, params, headers, payload.as_deref())?
        }
        Patch {
            endpoint,
            file,
            params,
            headers,
        } => {
            let params = if params.is_empty() {
                None
            } else {
                Some(params.into())
            };
            let headers = if headers.is_empty() {
                None
            } else {
                Some(headers.into())
            };
            let payload = if let Some(file) = file {
                Some(fs::read_to_string(file)?)
            } else {
                None
            };
            api.patch_raw(&endpoint, params, headers, payload.as_deref())?
        }
        Delete {
            endpoint,
            params,
            headers,
        } => {
            let params = if params.is_empty() {
                None
            } else {
                Some(params.into())
            };
            let headers = if headers.is_empty() {
                None
            } else {
                Some(headers.into())
            };
            api.delete_raw(&endpoint, params, headers)?
        }
    };
    println!("{}", response.body());
    Ok(())
}
