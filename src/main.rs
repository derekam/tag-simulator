use crate::simulation::Simulation;
use clap::{App, Arg, ArgMatches};
use crate::parameters::{TagParams, DEFAULT_PARAMS};
use std::fmt::Debug;

mod agent;
mod environment;
mod position;
mod tag_environment;
mod action;
mod parameters;
mod simulation;

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
        .arg(Arg::with_name("iterations")
            .short("i")
            .long("iterations")
            .takes_value(true)
            .help("The number of iterations to run of the simulation."))
        .get_matches();

    log::info!("Starting up Tag Simulator.");

    let parameters: TagParams = TagParams {
        speed: extract("speed", &matches, DEFAULT_PARAMS.speed),
        proximity: extract("proximity", &matches, DEFAULT_PARAMS.proximity),
        width: extract("width", &matches, DEFAULT_PARAMS.width),
        height: extract("height", &matches, DEFAULT_PARAMS.height),
        num_players: extract("num_players", &matches, DEFAULT_PARAMS.num_players)
    };

    let iterations: u128 = extract("iterations", &matches, 0);

    let mut simulation: Simulation = Simulation::new(parameters);
    let num_steps: Option<u128> = if iterations == 0 { Option::None } else { Option::Some(iterations) };
    simulation.run(num_steps);
    simulation.stop();
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
