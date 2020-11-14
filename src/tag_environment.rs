use dashmap::DashMap;
use crate::action::Action;
use crate::environment::Environment;
use crate::agent::Agent;
use std::borrow::Borrow;

pub struct TagEnvironment {
    pub(crate) agents: DashMap<usize, Agent>,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Environment<Action, Agent> for TagEnvironment {

    fn reset(&mut self) {
        self.agents.clear();
    }

    fn add_agent(&mut self, agent: Agent) {
        match &self.agents.borrow().insert(agent.id, agent) {
            None => {
                log::debug!("Agent {:?} added to the environment.", agent);
            }
            Some(agent) => {
                log::warn!("Agent {:?} already present in the environment. Agent was updated instead.", agent);
            }
        };
    }

    fn step(&mut self, agent: usize, action: &Action) {
        match action {
            Action::Tag(other) => {
                self.agents.get_mut(&agent).unwrap().is_it = false;
                self.agents.get_mut(&other).unwrap().last_tagged = agent;
                self.agents.get_mut(&other).unwrap().is_it = true;
                log::debug!("Agent {:?} has tagged agent {:?}.", agent, other)
            }
            Action::Move(position) => {
                self.agents.get_mut(&agent).unwrap().update(position);
            }
        }
    }

    fn step_all(&mut self, actions: Vec<Action>) {
        for (index, act) in actions.iter().enumerate() {
            self.step(index, act)
        }
    }

    fn render(&mut self) {
        // TODO either here or as part of Simulation.
    }

    fn shutdown(&mut self) {
        self.reset();
    }

}

#[cfg(test)]
mod tests {
    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::agent::Agent;
    use crate::environment::Environment;
    use crate::action::Action;

    #[test]
    fn can_setup_env() {
        let mut env: TagEnvironment = base_env();
        assert_eq!(3, env.agents.len());
        let agent2_dupe: Agent = Agent {
            id: 2,
            is_it: true,
            last_tagged: 1,
            position_x: 1.5,
            position_y: 1.0,
            speed: 1.0,
            reach: 2.0
        };
        env.add_agent(agent2_dupe);
        assert_eq!(3, env.agents.len());
        let new_agent = *env.agents.get(&2).unwrap().value();
        assert_eq!(agent2_dupe, new_agent);
        env.reset();
        assert_eq!(0, env.agents.len());
        env.shutdown();
    }

    #[test]
    fn updates_properly() {
        let mut env: TagEnvironment = base_env();
        let mut actions: Vec<Action> = Vec::with_capacity(env.agents.len());
        for agent in 0..env.agents.len() {
            actions.insert(agent, env.agents.get(&agent).unwrap().act(&env));
        }
        env.step_all(actions);
        env.render();
        assert_ne!(true, env.agents.get(&0).unwrap().is_it);
    }

    fn base_env() -> TagEnvironment {
        let mut env: TagEnvironment = TagEnvironment {
            agents: DashMap::with_capacity(3),
            width: 2.,
            height: 2.
        };
        let agent0: Agent = Agent {
            id: 0,
            is_it: true,
            last_tagged: 0,
            position_x: 0.0,
            position_y: 0.0,
            speed: 2.0,
            reach: 2.0
        };
        let agent1: Agent = Agent {
            id: 1,
            is_it: false,
            last_tagged: 1,
            position_x: 0.5,
            position_y: 0.5,
            speed: 2.0,
            reach: 2.0
        };
        let agent2: Agent = Agent {
            id: 2,
            is_it: false,
            last_tagged: 2,
            position_x: 1.0,
            position_y: 1.0,
            speed: 2.0,
            reach: 2.0
        };
        env.add_agent( agent0);
        env.add_agent( agent1);
        env.add_agent( agent2);
        env
    }

}