use std::io::{self, Read};
use atty::Stream;
use clap::{Arg, App};
use log::LevelFilter;
use std::fs::File;
use std::io::stdout;
use fern::Dispatch;
use tokio::runtime::Runtime;
use async_trait::async_trait;
use quick_js::{Context, JsValue};

mod analysis;
mod basis_network;
mod basis_node;
mod basis_graph;
mod config;
mod context;
mod data_node;
mod document;
mod document_format;
mod document_profile;
mod environment;
mod hash;
mod id;
mod lineage;
mod macros;
mod model;
mod normalization;
mod organization;
mod provider;
mod runtimes;
mod transformation;
mod translation;
mod types;
mod prelude;
mod utility;
mod json_node;

use crate::prelude::*;
use crate::config::{CONFIG};
use crate::document_profile::DocumentProfile;
use crate::provider::Provider;

fn load_stdin() -> io::Result<String> {
    log::trace!("In load_stdin");

    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

fn init_logging() {
    let path = format!("{}/{}", read_lock!(CONFIG).dev.debug_dir, "debug.log");
    let log_file = File::create(path).expect("Could not create log file");

    Dispatch::new()
        .level(LevelFilter::Off)
        .level_for("parversion", LevelFilter::Trace)
        .chain(stdout())
        .chain(log_file)
        .apply()
        .expect("Could not initialize logging");
}





struct TestProvider;

#[async_trait]
impl Provider for TestProvider {

    async fn get_document_profile(&self, features: &str) -> Result<Option<DocumentProfile>, Errors> {

        unimplemented!()

    }

}






#[tokio::main]
async fn main() {
    init_logging();

    let mut document = String::new();

    match load_stdin() {
        Ok(stdin) => {
            document = stdin;
        }
        Err(_e) => {
            log::debug!("Did not receive input from stdin");
        }
    }

    let matches = App::new("parversion")
        .arg(Arg::with_name("file")
             .short('f')
             .long("file")
             .value_name("FILE")
             .help("Provide file as document for processing"))
        .arg(Arg::with_name("basis")
             .short('b')
             .long("basis")
             .value_name("BASIS")
             .help("Provide basis graph"))
        .arg(Arg::with_name("format")
            .short('o')
            .long("output-format")
            .value_name("FORMAT")
            .help("Set output format: JSON, HTML, XML, text"))
        .arg(Arg::with_name("url")
            .short('u')
            .long("url")
            .value_name("URL")
            .help("The full URL that identifies and locates the provided document"))
        .get_matches();

    let origin: Option<String> = matches.value_of("url").map(|s| s.to_string());



    let context = Context::new().unwrap();
    let value = context.eval("1 + 2").unwrap();
    assert_eq!(value, JsValue::Int(3));







    let provider = TestProvider;

    let options = Options {
        origin,
        ..Options::default()
    };



    let analysis = match matches.value_of("file") {
        Some(path) => {
            normalization::normalize_file_to_analysis(
                &provider,
                path,
                &Some(options),
            ).await.expect("Could not normalize file")
        }
        None => {
            log::info!("File not provided");
            normalization::normalize_text_to_analysis(
                &provider,
                document,
                &Some(options),
            ).await.expect("Could not normalize text")
        }
    };



    let basis_graph = analysis.build_basis_graph();



    let document_format = document_format::DocumentFormat::default();


    let normalized_text = analysis.to_document(&Some(document_format)).expect("Could not convert to document").to_string();


    println!("{}", normalized_text);





    unimplemented!()
}
