use crate::agents::agent_type::AgentType;

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

    /// What type of agent the players should use.
    pub agent_type: AgentType,

    /// Whether the rendered players should be numbered (has a performance cost).
    pub numbered: bool,

}

const SPEED: f64 = 5.0;
const PROXIMITY: f64 = 15.0;
const WIDTH: u64 = 1000;
const HEIGHT: u64 = 600;
const NUM_PLAYERS: usize = 50;

pub(crate) const DEFAULT_PARAMS: TagParams = TagParams {
    speed: SPEED,
    proximity: PROXIMITY,
    width: WIDTH,
    height: HEIGHT,
    num_players: NUM_PLAYERS,
    agent_type: AgentType::Default,
    numbered: false
};
