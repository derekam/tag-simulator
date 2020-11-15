use crate::environment::Environment;
use crate::parameters::{TagParams};
use crate::tag_environment::TagEnvironment;
use dashmap::DashMap;
use crate::action::Action;
use crate::controls::Controls;
use iced::{Application, Settings, window};
use crate::agents::agent::Agent;
use std::collections::HashSet;

/// The main tag simulation instance.
///
/// # Examples
/// ## Standalone (no UI)
/// ```
///     let simulation: Simulation = Simulation::new(DEFAULT_PARAMS);
///     simulation.run_headless(Option::from(500));
/// ```
///
/// ## WIth Iced GUI
/// ```
///     Simulation::run_gui(DEFAULT_PARAMS);
/// ```
pub struct Simulation<X>
    where
        X: Agent + 'static
{
    pub(crate) parameters: TagParams,
    pub(crate) environment: TagEnvironment<X>,
    pub(crate) is_running: bool,
    pub(crate) controls: Controls,
}

impl<X: Agent + 'static> Simulation<X> {

    pub fn new(parameters: TagParams) -> Self {
        let mut sim = Simulation {
                    parameters,
                    environment: TagEnvironment {
                        agents: DashMap::with_capacity(parameters.speed as usize),
                        width: parameters.width as f32,
                        height: parameters.height as f32,
                        it: HashSet::new(),
                        show_numbers: parameters.numbered,
                    },
                    is_running: false,
                    controls: Controls::default(),
                };
                sim.environment.reset(parameters);
                sim

    }

    pub fn run_gui(parameters: TagParams) {
        let window = window::Settings {
            size: (parameters.width as u32, parameters.height as u32),
            resizable: false,
            decorations: true
        };
        let settings = Settings {
            window,
            flags: parameters,
            default_font: None,
            antialiasing: true
        };
        Simulation::<X>::run(settings);
    }

    pub fn run_headless(&mut self, num_steps: Option<u128>) {
        match num_steps {
            None => {
                self.is_running = true;
                while self.is_running {
                    self.step();
                }
            }
            Some(steps) => {
                self.is_running = true;
                for _ in 0..steps {
                    self.step();
                }
                self.is_running = false;
            }
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub(crate) fn step(&mut self) {
        // TODO something like a countdown latch here or Rayon iters, or abandon turn-based altogether and have agents in their own threads.
        let mut actions: Vec<Action> = Vec::with_capacity(self.environment.agents.len());
        for agent in 0..self.environment.agents.len() {
            actions.insert(agent, self.environment.agents.get(&agent).unwrap().act(&self.environment));
        }
        &self.environment.step_all(actions);
    }

}

#[cfg(test)]
mod tests {
    use crate::parameters::TagParams;
    use crate::simulation::Simulation;
    use crate::agents::agent_type::AgentType;
    use crate::agents::agent::{Player};
    use crate::agents::basic_directional::DirectionalAgent;
    use test::Bencher;

    #[test]
    fn test_basic_functionality() {
        let params: TagParams = TagParams {
            speed: 10.0,
            proximity: 2.0,
            width: 100,
            height: 100,
            num_players: 5,
            agent_type: AgentType::Default,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<Player> = Simulation::new(params);
        assert_eq!(sim.is_running, false);
        assert_eq!(sim.environment.height, 100.);
        assert_eq!(sim.environment.width, 100.);
        assert_eq!(sim.environment.agents.len(), 5);
        let agent = sim.environment.agents.get(&0).unwrap().value().clone();
        sim.run_headless(Option::from(10));
        assert_ne!(agent.position, sim.environment.agents.get(&0).unwrap().position);
    }

    #[bench]
    fn bench_headless_500_directional(b: &mut Bencher) {
        let params: TagParams = TagParams {
            speed: 5.0,
            proximity: 2.0,
            width: 1000,
            height: 600,
            num_players: 500,
            agent_type: AgentType::BasicDirectional,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<DirectionalAgent> = Simulation::new(params);
        b.iter(|| {
            sim.step();
        });
    }

    #[bench]
    fn bench_headless_5000_directional(b: &mut Bencher) {
        let params: TagParams = TagParams {
            speed: 5.0,
            proximity: 2.0,
            width: 1000,
            height: 600,
            num_players: 5000,
            agent_type: AgentType::BasicDirectional,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<DirectionalAgent> = Simulation::new(params);
        b.iter(|| {
            sim.step();
        });
    }

    #[bench]
    fn bench_headless_500_default(b: &mut Bencher) {
        let params: TagParams = TagParams {
            speed: 5.0,
            proximity: 2.0,
            width: 1000,
            height: 600,
            num_players: 500,
            agent_type: AgentType::Default,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<Player> = Simulation::new(params);
        b.iter(|| {
            sim.step();
        });
    }


    #[bench]
    fn bench_headless_5000_default(b: &mut Bencher) {
        let params: TagParams = TagParams {
            speed: 5.0,
            proximity: 2.0,
            width: 1000,
            height: 600,
            num_players: 5000,
            agent_type: AgentType::Default,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<Player> = Simulation::new(params);
        b.iter(|| {
            sim.step();
        });
    }

    #[bench]
    fn bench_headless_50000_default(b: &mut Bencher) {
        let params: TagParams = TagParams {
            speed: 5.0,
            proximity: 2.0,
            width: 1000,
            height: 600,
            num_players: 50000,
            agent_type: AgentType::Default,
            numbered: false,
            num_it: 1
        };
        let mut sim: Simulation<Player> = Simulation::new(params);
        b.iter(|| {
            sim.step();
        });
    }

}