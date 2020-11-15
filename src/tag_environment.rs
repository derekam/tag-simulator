use dashmap::DashMap;
use crate::action::Action;
use crate::environment::Environment;
use crate::agent::Agent;
use std::borrow::Borrow;
use iced::{canvas, Point, Color, HorizontalAlignment, VerticalAlignment};
use iced::canvas::Path;

#[derive(Debug, Clone)]
pub struct TagEnvironment {
    pub(crate) agents: DashMap<usize, Agent>,
    pub(crate) width: f32,
    pub(crate) height: f32,
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
                log::info!("Agent {:?} has tagged agent {:?}.", agent, other)
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

}

impl canvas::Drawable for TagEnvironment {

    fn draw(&self, frame: &mut canvas::Frame) {

        let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

        let tagged_agents = Path::new(|path| {
            for agent in &self.agents {
                frame.fill_text(canvas::Text {
                    content: agent.id.to_string(),
                    position: agent.position,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                    size: 15.0,
                    ..canvas::Text::default()
                });
                if agent.is_it {
                    path.circle(agent.position, agent.reach);
                }
            }
        });

        let agents = Path::new(|path| {
            for agent in &self.agents {
                if !agent.is_it {
                   path.circle(agent.position, agent.reach);
                }
            }
        });

        frame.fill(&space, Color::BLACK);
        frame.fill(&agents, Color::WHITE);
        frame.fill(&tagged_agents, Color::from_rgb8(0xF9, 0xD7, 0x1C));

    }

}


#[cfg(test)]
mod tests {
    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::agent::Agent;
    use crate::environment::Environment;
    use crate::action::Action;
    use iced::Point;

    #[test]
    fn can_setup_env() {
        let mut env: TagEnvironment = base_env();
        assert_eq!(3, env.agents.len());
        let agent2_dupe: Agent = Agent {
            id: 2,
            is_it: true,
            last_tagged: 1,
            position: Point {
                x: 1.5,
                y: 1.0,
            },
            speed: 1.0,
            reach: 2.0
        };
        env.add_agent(agent2_dupe);
        assert_eq!(3, env.agents.len());
        let new_agent = *env.agents.get(&2).unwrap().value();
        assert_eq!(agent2_dupe, new_agent);
        env.reset();
        assert_eq!(0, env.agents.len());
    }

    #[test]
    fn updates_properly() {
        let mut env: TagEnvironment = base_env();
        let mut actions: Vec<Action> = Vec::with_capacity(env.agents.len());
        for agent in 0..env.agents.len() {
            actions.insert(agent, env.agents.get(&agent).unwrap().act(&env));
        }
        env.step_all(actions);
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
            position: Point {
                x: 0.0,
                y: 0.0,
            },
            speed: 2.0,
            reach: 2.0
        };
        let agent1: Agent = Agent {
            id: 1,
            is_it: false,
            last_tagged: 1,
            position: Point {
                x: 0.5,
                y: 0.5,
            },
            speed: 2.0,
            reach: 2.0
        };
        let agent2: Agent = Agent {
            id: 2,
            is_it: false,
            last_tagged: 2,
            position: Point {
                x: 1.0,
                y: 1.0,
            },
            speed: 2.0,
            reach: 2.0
        };
        env.add_agent( agent0);
        env.add_agent( agent1);
        env.add_agent( agent2);
        env
    }

}