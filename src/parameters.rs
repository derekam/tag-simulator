#[derive(Clone, Copy)]
pub struct TagParams {

    /// The maximum distance per turn of a player/agent.
    pub speed: f64,

    /// The distance within which one player/agent may tag another.
    pub proximity: f64,

    /// The width of the field.
    pub width: u64,

    /// The height of the field.
    pub height: u64,

    /// The number of players/agents.
    pub num_players: usize,

}

const SPEED: f64 = 2.0;
const PROXIMITY: f64 = 1.0;
const WIDTH: u64 = 50;
const HEIGHT: u64 = 50;
const NUM_PLAYERS: usize = 10;

pub(crate) const DEFAULT_PARAMS: TagParams = TagParams {
    speed: SPEED,
    proximity: PROXIMITY,
    width: WIDTH,
    height: HEIGHT,
    num_players: NUM_PLAYERS,
};
