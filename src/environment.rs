use crate::parameters::TagParams;

/// The main simulation environment trait.
/// This was originally created with the intention of
///     possible interop with Python as an OpenAI Gym Env,
///     but wound up out of scope.
pub trait Environment<ACTION, AGENT> {

    fn reset(&mut self, params: TagParams);

    fn add_agent(&mut self, agent: AGENT);

    fn step(&mut self, agent: usize, action: &ACTION);

    fn step_all(&mut self, actions: Vec<ACTION>);

}