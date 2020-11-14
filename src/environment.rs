/// The main simulation environment trait.
pub trait Environment<ACTION, AGENT> {

    fn reset(&mut self);

    fn add_agent(&mut self, agent: AGENT);

    fn step(&mut self, agent: usize, action: &ACTION);

    fn step_all(&mut self, actions: Vec<ACTION>);

    fn render(&mut self);

    fn shutdown(&mut self);

}