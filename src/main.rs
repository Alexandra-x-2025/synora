mod cli;
mod domain;
mod integration;
mod logging;
mod paths;
mod repository;
mod security;
mod service;

fn main() {
    std::process::exit(cli::run());
}
