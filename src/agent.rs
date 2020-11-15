use crate::tag_environment::TagEnvironment;
use crate::action::{Action};
use rand::{thread_rng, Rng};
use iced::Point;

/// A simplistic agent for playing tag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Agent {
    pub id: usize,
    pub is_it: bool,
    pub last_tagged: usize,
    pub position: Point,
    pub speed: f32,
    pub reach: f32,
}

impl Agent {

    /// Action selection for a player agent.
    /// This is overly simplistic -- it just tags any eligible players if 'it' and others nearby,
    /// and moves in a random direction otherwise.
    /// TODO: Abstract out this functionality into a trait and implement different strategies.
    pub(crate) fn act(&self, env: &TagEnvironment) -> Action {

        if self.is_it {
            for agent in &env.agents {
                if !agent.is_it && agent.id != self.id &&
                    agent.id != self.last_tagged &&
                    self.distance(*agent) <= self.reach {
                    return Action::Tag(agent.id)
                }
            }
        }

        self.random_move(env)
    }

    /// Updates the position of an agent.
    pub fn update(&mut self, position: &Point) {
        self.position = *position;
    }

    /// Cartesian distance between two agents.
    fn distance(&self, other: Agent) -> f32 {
        return ((self.position.x - other.position.x).abs().powf(2.) + (self.position.y - other.position.y).abs().powf(2.)).sqrt()
    }

    /// Create and return a move action in a random direction.
    fn random_move(&self, env: &TagEnvironment) -> Action {
        let t: f32 = thread_rng().gen::<f32>() * std::f32::consts::PI * 2.0;
        let u: f32 = thread_rng().gen::<f32>() + thread_rng().gen::<f32>();
        let r = if u > 1.0 { 1.0 - u } else { u };
        let x = r * t.cos() * self.speed;
        let y = r * t.sin() * self.speed;
        Action::Move(Point {
            x: f32::max(0.0, f32::min(env.width, self.position.x + x)),
            y: f32::max(0.0, f32::min(env.height, self.position.y + y)),
        })
     }

}

#[cfg(test)]
mod tests {

    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::agent::Agent;
    use crate::action::{Action};
    use crate::action::Action::Tag;
    use iced::Point;

    #[test]
    fn no_tag_backs() {
        let env: TagEnvironment = TagEnvironment {
            agents: DashMap::with_capacity(2),
            width: 2.,
            height: 2.
        };
        let mut agent1: Agent = Agent {
            id: 1,
            is_it: true,
            last_tagged: 1,
            position: Point {
                x: 0.0,
                y: 0.0
            },
            speed: 2.0,
            reach: 2.0
        };
        let agent2: Agent = Agent {
            id: 2,
            is_it: false,
            last_tagged: 2,
            position: Point {
                x: 0.0,
                y: 0.0
            },
            speed: 2.0,
            reach: 2.0
        };
        env.agents.insert(1, agent1);
        env.agents.insert(2, agent2);
        let tag: Action = agent1.act(&env);
        assert_eq!(tag, Tag(2), "Making sure an agent tags another in-range agent when 'it'");
        agent1.last_tagged = 2;
        let tag: Action = agent1.act(&env);
        assert_ne!(tag, Tag(2), "Making sure an agent does not tag-back the one that tagged it");
    }

}
