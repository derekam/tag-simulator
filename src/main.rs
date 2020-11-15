#![feature(test)]

extern crate test;

use crate::simulation::Simulation;
use clap::{App, Arg, ArgMatches};
use crate::parameters::{TagParams, DEFAULT_PARAMS};
use std::fmt::Debug;
use crate::agents::agent_type::AgentType;
use crate::agents::agent::Player;
use crate::agents::basic_directional::DirectionalAgent;

mod environment;
mod tag_environment;
mod action;
mod parameters;
mod simulation;
mod iced_ui;
mod controls;
mod time;
mod agents;

fn main() {
    env_logger::init();
    let matches: ArgMatches = App::new("Tag Simulator")
        .version("0.1.0")
        .author("Derek A <self@derekammerman.com>")
        .about("An agent-based simulation engine for the game of tag.")
        .arg(Arg::with_name("speed")
            .short("s")
            .long("speed")
            .takes_value(true)
            .help("The maximum distance per turn of a player/agent"))
        .arg(Arg::with_name("proximity")
            .short("p")
            .long("proximity")
            .takes_value(true)
            .help("The distance within which one player/agent may tag another"))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true)
            .help("The width of the field"))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .takes_value(true)
            .help("The height of the field"))
        .arg(Arg::with_name("num_players")
            .short("n")
            .long("num_players")
            .takes_value(true)
            .help("The number of players/agents"))
        .arg(Arg::with_name("directional_agent")
            .short("d")
            .long("directional_agent")
            .takes_value(false)
            .help("Have players/agents use a simple directional strategy."))
        .arg(Arg::with_name("text_numbers")
            .short("t")
            .long("text_numbers")
            .takes_value(false)
            .help("Whether the players should be numbered in the UI."))
        .get_matches();

    log::info!("Starting up Tag Simulator.");

    let agent_type = if matches.is_present("directional_agent") { AgentType::BasicDirectional } else { AgentType::Default };

    let parameters: TagParams = TagParams {
        speed: extract("speed", &matches, DEFAULT_PARAMS.speed),
        proximity: extract("proximity", &matches, DEFAULT_PARAMS.proximity),
        width: extract("width", &matches, DEFAULT_PARAMS.width),
        height: extract("height", &matches, DEFAULT_PARAMS.height),
        num_players: extract("num_players", &matches, DEFAULT_PARAMS.num_players),
        agent_type,
        numbered: matches.is_present("text_numbers")
    };

    match parameters.agent_type {
        AgentType::Default => {
            Simulation::<Player>::run_gui(parameters);
        },
        AgentType::BasicDirectional => {
            Simulation::<DirectionalAgent>::run_gui(parameters);
        }
    }

}

fn extract<TYPE: Debug + std::str::FromStr>(name: &str, args: &ArgMatches, default: TYPE) -> TYPE {
    match args.value_of(name) {
        None => {
            log::info!("Using default value of {:?} for {:?}.", default, name);
            default
        }
        Some(value) => {
            match value.parse::<TYPE>() {
                Ok(num) => {
                    num
                }
                Err(_) => {
                    log::warn!("Value {:?} passed for {:?} is invalid. Using default of {:?}.", value, name, default);
                    default
                }
            }
        }
    }
}
