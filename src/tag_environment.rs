use crate::action::Action;
use crate::environment::Environment;
use std::borrow::Borrow;
use iced::{canvas, Point, Color, HorizontalAlignment, VerticalAlignment};
use iced::canvas::{Path, Frame};
use rand::{thread_rng, Rng};
use crate::parameters::TagParams;
use crate::agents::agent::Agent;
use iced::widget::canvas::Layer;
use async_std::sync::Arc;
use iced_native::Size;
use iced_wgpu::Primitive;
use dashmap::DashMap;
use std::collections::HashSet;

/// The state of the environment of the simulation.
#[derive(Debug, Clone)]
pub struct TagEnvironment<P>
where
    P: Agent
{
    // Just having a map like this really isn't ideal;
    //  this should be broken into a grid of buckets,
    //  or something like an R* tree.
    pub(crate) agents: DashMap<usize, P>,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) it: HashSet<usize>,
    pub(crate) show_numbers: bool,
}

impl<P> Environment<Action, P> for TagEnvironment<P>
    where
        P: Agent
{

    fn reset(&mut self, params: TagParams) {
        self.show_numbers = params.numbered;
        self.agents.clear();
        let mut rng = thread_rng();

        for agent in 0..params.num_players {
            self.add_agent(P::create(agent, params))
        };

        self.it.clear();
        let mut to_pick = params.num_it;
        while to_pick > 0 {
            let it: usize = rng.gen_range(0, params.num_players);
            if self.it.insert(it) {
                log::info!("Starting with player {:?} marked it.", it);
                self.agents.get_mut(&it).unwrap().tag(it);
                to_pick -= 1;
            }
        }

    }

    fn add_agent(&mut self, agent: P) {
        match &self.agents.borrow().insert(agent.player().id, agent) {
            None => {
                log::debug!("Agent {:?} added to the environment.", &agent);
            }
            Some(agent) => {
                log::warn!("Agent {:?} already present in the environment. Agent was updated instead.", &agent);
            }
        };
    }

    fn step(&mut self, agent: usize, action: &Action) {
        log::debug!("Applying action {:?} from agent {:?}", action, agent);
        match action {
            Action::Tag(other) => {
                if self.it.insert(*other) {
                    self.agents.get_mut(&agent).unwrap().untag();
                    self.agents.get_mut(&other).unwrap().tag(agent);
                    self.it.remove(&agent);
                    log::info!("Agent {:?} has tagged agent {:?}.", agent, other)
                } else {
                    log::info!("{:?} has already been tagged.", *other);
                }
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

impl<P> Layer for TagEnvironment<P>
    where
        P: Agent
{
    fn draw(&self, bounds: Size) -> Arc<Primitive> {
        let mut frame = Frame::new(bounds.width, bounds.height);
        self.draw_frame(&mut frame);
        Arc::new(frame.into_primitive())
    }

}

impl<P> TagEnvironment<P>
    where
        P: Agent
{

    fn draw_frame(&self, frame: &mut canvas::Frame) {
        let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());
        frame.fill(&space, Color::BLACK);

        for agent in &self.agents {
            if self.show_numbers {
                frame.fill_text(canvas::Text {
                    content: agent.player().id.to_string(),
                    position: agent.player().position,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                    size: 15.0,
                    ..canvas::Text::default()
                });
            }
            if !agent.player().is_it {
                frame.fill(&Path::circle(agent.player().position, agent.player().reach), Color::WHITE);
            } else {
                frame.fill(&Path::circle(agent.player().position, agent.player().reach), Color::from_rgb8(0xF9, 0xD7, 0x1C));
            }
        }

    }

}


#[cfg(test)]
mod tests {
    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::environment::Environment;
    use crate::action::Action;
    use iced::Point;
    use crate::agents::agent::{Player, Agent};
    use crate::parameters::DEFAULT_PARAMS;
    use std::collections::HashSet;
    use crate::action::Action::Tag;

    #[test]
    fn can_setup_env() {
        let mut env: TagEnvironment<Player> = base_env();
        assert_eq!(3, env.agents.len());
        let agent2_dupe: Player = Player {
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
        assert_eq!(&agent2_dupe, env.agents.get(&2).unwrap().value());
        &env.reset(DEFAULT_PARAMS);
        assert_eq!(DEFAULT_PARAMS.num_players, env.agents.len());
    }

    #[test]
    fn updates_properly() {
        let mut env: TagEnvironment<Player> = base_env();
        let mut actions: Vec<Action> = Vec::with_capacity(env.agents.len());
        for agent in 0..env.agents.len() {
            actions.insert(agent, env.agents.get(&agent).unwrap().act(&env));
        }
        env.step_all(actions);
        assert_ne!(true, env.agents.get(&0).unwrap().is_it);
    }

    fn base_env() -> TagEnvironment<Player> {
        let mut env: TagEnvironment<Player> = TagEnvironment {
            agents: DashMap::with_capacity(3),
            width: 2.,
            height: 2.,
            it: HashSet::new(),
            show_numbers: false
        };
        let agent0: Player = Player {
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
        let agent1: Player = Player {
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
        let agent2: Player = Player {
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
        env.it.insert(0);
        env.add_agent( agent0);
        env.add_agent( agent1);
        env.add_agent( agent2);
        env
    }

    #[test]
    fn multiple_tag_same() {
        let mut env: TagEnvironment<Player> = base_env();
        let act = Tag(2);
        env.step(1, &act);
        assert_eq!(2, env.it.len());
        env.step(0, &act);
        assert_eq!(2, env.it.len());
    }

}