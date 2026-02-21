mod cli;
mod domain;
mod integration;
mod repository;
mod security;
mod service;

fn main() {
    std::process::exit(cli::run());
}
