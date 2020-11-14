use crate::environment::Environment;
use crate::parameters::{TagParams};
use crate::tag_environment::TagEnvironment;
use dashmap::DashMap;
use crate::agent::{Agent};
use rand::{thread_rng, Rng};
use crate::action::Action;

/// The main tag simulator,
/// to be extended with visualization.
/// TODO example usage documenation
pub struct Simulation {
    environment: TagEnvironment,
    is_running: bool,
}

impl Simulation {

    pub fn new(parameters: TagParams) -> Self {
        let mut rng = thread_rng();

        let mut sim = Simulation {
            environment: TagEnvironment {
                agents: DashMap::with_capacity(parameters.speed as usize),
                width: parameters.width as f64,
                height: parameters.height as f64
            },
            is_running: false,
        };
        for agent in 0..parameters.num_players {
            sim.environment.add_agent(Agent {
                id: agent,
                is_it: false,
                last_tagged: agent,
                position_x: rng.gen_range(0.0, parameters.width as f64),
                position_y: rng.gen_range(0.0, parameters.height as f64),
                speed: parameters.speed,
                reach: parameters.proximity,
            })
        };

        let it: usize = rng.gen_range(0, parameters.num_players);
        sim.environment.agents.get_mut(&it).unwrap().is_it = true;
        sim
    }

    pub fn run(&mut self, num_steps: Option<u128>) {
        match num_steps {
            None => {
                self.is_running = true;
                while self.is_running {
                    self.step();
                }
            }
            Some(steps) => {
                for _ in 0..steps {
                    self.step();
                }
            }
        }

        self.environment.shutdown();
    }

    fn step(&mut self) {
        let mut actions: Vec<Action> = Vec::with_capacity(self.environment.agents.len());
        for agent in 0..self.environment.agents.len() {
            actions.insert(agent, self.environment.agents.get(&agent).unwrap().act(&self.environment));
        }
        &self.environment.step_all(actions);
        self.environment.render()
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_functionality() {
        // TODO after restructuring for view
    }

}