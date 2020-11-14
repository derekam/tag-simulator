use crate::tag_environment::TagEnvironment;
use crate::action::Action;
use crate::position::Position;
use rand::{thread_rng, Rng};

/// A simplistic agent for playing tag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Agent {
    pub id: usize,
    pub is_it: bool,
    pub last_tagged: usize,
    pub position_x: f64,
    pub position_y: f64,
    pub speed: f64,
    pub reach: f64,
}

impl Agent {

    /// Action selection for a player agent.
    /// This is overly simplistic -- it just tags any eligible players if 'it' and others nearby,
    /// and moves in a random direction otherwise.
    /// TODO: Abstract out this functionality into a trait and implement different strategies.
    pub(crate) fn act(&self, env: &TagEnvironment) -> Action {

        if self.is_it {
            for agent in &env.agents {
                // TODO clean this up
                if !agent.is_it && agent.id != self.id &&
                    agent.id != self.last_tagged && self.distance(*agent) <= self.reach {
                    return Action::Tag(agent.id)
                }
            }
        }

        let x_new = thread_rng().gen_range(f64::max(0.0, self.position_x - self.speed), f64::min(env.width, self.position_x + self.speed));
        let remaining = (x_new - self.position_x).abs();
        let y_new = thread_rng().gen_range(f64::max(0.0, self.position_y - remaining), f64::min(env.height, self.position_y + remaining));

        Action::Move(Position {
            x: x_new,
            y: y_new
        })
    }

    /// Updates the position of an agent.
    pub fn update(&mut self, position: &Position) {
        self.position_x = position.x;
        self.position_y = position.y;
    }

    /// Cartesian distance between two agents.
    fn distance(&self, other: Agent) -> f64 {
        return ((self.position_x - other.position_x).abs().powf(2.) + (self.position_y - other.position_y).abs().powf(2.)).sqrt()
    }

}

#[cfg(test)]
mod tests {

    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::agent::Agent;
    use crate::action::Action;
    use crate::action::Action::Tag;

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
            position_x: 0.0,
            position_y: 0.0,
            speed: 2.0,
            reach: 2.0
        };
        let agent2: Agent = Agent {
            id: 2,
            is_it: false,
            last_tagged: 2,
            position_x: 0.0,
            position_y: 0.0,
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
