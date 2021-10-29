use std::{
    fs,
    io::{stdout, Write},
    net::SocketAddrV4,
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
            rest_api_sub_command(subcmd, api)?;
        }
        SubCommand::DefaultIniPath => {
            ini::default_location().map(|p| stdout().write(p.as_os_str().as_bytes()));
        }
    }

    Ok(())
}

fn rest_api_sub_command(subcmd: RestApiSubCommand, api: RestApi) -> Result<(), Error> {
    use RestApiSubCommand::*;
    match subcmd {
        RestApiSubCommand::SuiteQl {
            query,
            limit,
            offset,
        } => {
            let resp = api.suiteql.raw(&query, limit, offset)?;
            println!("{}", &resp.body());
        }
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
            let resp = api.get_raw(&endpoint, params, headers)?;
            println!("{}", &resp.body());
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
            let resp = api.post_raw(&endpoint, params, headers, payload.as_deref())?;
            println!("{}", &resp.body());
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
            let resp = api.put_raw(&endpoint, params, headers, payload.as_deref())?;
            println!("{}", &resp.body());
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
            let resp = api.patch_raw(&endpoint, params, headers, payload.as_deref())?;
            println!("{}", &resp.body());
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
            let resp = api.delete_raw(&endpoint, params, headers)?;
            println!("{}", &resp.body());
        }
        JsonSchema { record_type } => {
            let resp = api.metadata.jsonschema(&record_type)?;
            println!("{}", &resp.body());
        }
        OpenApi { record_types } => {
            let resp = api.metadata.openapi(&record_types)?;
            println!("{}", &resp.body());
        }
        OpenApiServe { record_types, bind } => {
            let resp = api.metadata.openapi(&record_types)?;
            let schema = resp.body();
            openapi_serve(bind, schema)?
        }
    };
    Ok(())
}

fn openapi_serve(bind: SocketAddrV4, schema: &str) -> Result<(), Error> {
    use tiny_http::{Header, Response, Server, StatusCode};
    let server = Server::http(bind).unwrap();
    for req in server.incoming_requests() {
        match (req.method(), req.url()) {
            (&tiny_http::Method::Get, "/") => {
                let mut resp = Response::from_string(include_str!("openapi-docs.html"));
                resp.add_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                );
                req.respond(resp)?;
            }
            (&tiny_http::Method::Get, "/openapi.json") => {
                let resp = Response::from_string(schema);
                req.respond(resp)?;
            }
            _ => {
                req.respond(Response::new_empty(StatusCode(404)))?;
            }
        }
    }
    Ok(())
}
