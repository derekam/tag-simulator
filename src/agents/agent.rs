use crate::tag_environment::TagEnvironment;
use crate::action::{Action};
use rand::{thread_rng, Rng};
use iced::Point;
use crate::parameters::TagParams;
use std::fmt::Debug;

/// A simplistic agent for playing tag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player {
    pub id: usize,
    pub is_it: bool,
    pub last_tagged: usize,
    pub position: Point,
    pub speed: f32,
    pub reach: f32,
}

pub trait Agent: Sized + Debug + Copy {
    
    fn act(&self, env: &TagEnvironment<Self>) -> Action;

    fn create(id: usize, params: TagParams) -> Self;

    fn update(&mut self, position: &Point);

    fn player(&self) -> Player;

    fn tag(&mut self, by: usize);

    fn untag(&mut self);

}

impl Agent for Player {

    /// Action selection for the default player agent.
    /// This is overly simplistic -- it just tags any eligible players if 'it' and others are nearby,
    /// and moves in a random direction otherwise.
    fn act(&self, env: &TagEnvironment<Player>) -> Action {
        if self.is_it {
            for agent in &env.agents {
                if !agent.is_it && agent.id != self.id &&
                    agent.id != self.last_tagged &&
                    self.distance(*agent) <= self.reach {
                    return Action::Tag(agent.id)
                }
            }
        }

        self.random_move(env.width, env.height)
    }

    fn create(id: usize, params: TagParams) -> Self {
        let mut rng = thread_rng();
        Player {
            id,
            is_it: false,
            last_tagged: id,
            position: Point {
                x: rng.gen_range(0.0, params.width as f32),
                y: rng.gen_range(0.0, params.height as f32),
            },
            speed: params.speed as f32,
            reach: params.proximity as f32,
        }
    }

    /// Updates the position of an agent.
    fn update(&mut self, position: &Point) {
        self.position = *position;
    }

    fn player(&self) -> Player {
        *self
    }

    fn tag(&mut self, by: usize) {
        self.is_it = true;
        self.last_tagged = by;
    }

    fn untag(&mut self) {
        self.is_it = false;
    }
}

impl Player {
    /// Cartesian distance between two agents.
    pub fn distance(&self, other: Player) -> f32 {
        return ((self.position.x - other.position.x).abs().powf(2.) + (self.position.y - other.position.y).abs().powf(2.)).sqrt()
    }

    // TODO this and move_away are messy/repetitive and need to be cleaned up
    pub fn move_towards(&self, other: Player, max_width: f32, max_height: f32) -> Action {
        log::debug!("{:?} is moving towards {:?}", self.id, other.id);
        let mut delta_x: f32 = other.position.x - self.position.x;
        let mut delta_y: f32 = other.position.y - self.position.y;
        while delta_y == 0. {
            delta_y = thread_rng().gen();
        }
        while delta_x == 0. {
            delta_x = thread_rng().gen();
        }
        let direction: f32 = (delta_y / delta_x).atan();
        let x = if delta_x < 0. { self.position.x - (self.speed * direction.cos()).abs() } else { self.position.x + (self.speed * direction.cos()).abs() };
        let y = if delta_y < 0. { self.position.y - (self.speed * direction.sin()).abs() } else { self.position.y + (self.speed * direction.sin()).abs() };
        Action::Move(
            Point {
                x: Player::clip(x, max_width),
                y: Player::clip(y, max_height)
            }
        )
    }

    /// Moves directly opposite to the line of sight between the player and the player to move away from.
    /// TODO allow more variance in the angle to move at so that it stops running to corners immediately.
    pub fn move_away(&self, other: Player, max_width: f32, max_height: f32) -> Action {
        log::debug!("{:?} is moving away from  {:?}", self.id, other.id);
        let mut delta_x: f32 = other.position.x - self.position.x;
        let mut delta_y: f32 = other.position.y - self.position.y;
        while delta_y == 0. {
            delta_y = thread_rng().gen();
        }
        while delta_x == 0. {
            delta_x = thread_rng().gen();
        }
        let direction: f32 = (-delta_y / -delta_x).atan();
        let x = if delta_x < 0. { self.position.x + (self.speed * direction.cos()).abs() } else { self.position.x - (self.speed * direction.cos()).abs() };
        let y =  if delta_y < 0. { self.position.y + (self.speed * direction.sin()).abs() } else { self.position.y - (self.speed * direction.sin()).abs() };
        Action::Move(
            Point {
                x: Player::clip(x, max_width),
                y: Player::clip(y, max_height)
            }
        )
    }

    /// Create and return a move action in a random direction.
    pub fn random_move(&self, max_width: f32, max_height: f32) -> Action {
        let t: f32 = thread_rng().gen::<f32>() * std::f32::consts::PI * 2.0;
        let u: f32 = thread_rng().gen::<f32>() + thread_rng().gen::<f32>();
        let r = if u > 1.0 { 1.0 - u } else { u };
        let x = r * t.cos() * self.speed;
        let y = r * t.sin() * self.speed;
        Action::Move(Point {
            x: f32::max(0.0, f32::min(max_width, self.position.x + x)),
            y: f32::max(0.0, f32::min(max_height, self.position.y + y)),
        })
     }

    fn clip(value: f32, max_value: f32) -> f32 {
        f32::max(0.0, f32::min(max_value, value))
    }

}

#[cfg(test)]
mod tests {

    use crate::tag_environment::TagEnvironment;
    use dashmap::DashMap;
    use crate::action::{Action};
    use crate::action::Action::{Tag, Move};
    use iced::Point;
    use crate::agents::agent::{Player, Agent};
    use crate::parameters::DEFAULT_PARAMS;

    #[test]
    fn no_tag_backs() {
        let env: TagEnvironment<Player> = TagEnvironment {
            agents: DashMap::with_capacity(2),
            width: 2.,
            height: 2.
        };
        let mut agent1: Player = Player {
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
        let agent2: Player = Player {
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

    #[test]
    pub fn move_towards() {
        let mut tagged = Player {
            id: 1,
            is_it: true,
            last_tagged: 1,
            position: Point {
                x: 0.0,
                y: 0.0
            },
            speed: 1.0,
            reach: 1.0
        };
        let mut untagged = Player {
            id: 2,
            is_it: true,
            last_tagged: 2,
            position: Point {
                x: 2.0,
                y: 2.0
            },
            speed: 1.0,
            reach: 1.0
        };
        
        let mut action: Action = tagged.move_towards(untagged, 3., 3.);
        let mut expected: Action = Move(Point {
            x: std::f32::consts::FRAC_1_SQRT_2,
            y: std::f32::consts::FRAC_1_SQRT_2
        });
        assert_eq!(expected, action);
        tagged.is_it = false;
        untagged.is_it = true;
        action = untagged.move_towards(tagged, 3., 3.);
        expected = Move(Point {
            x: 2. - std::f32::consts::FRAC_1_SQRT_2,
            y: 2. - std::f32::consts::FRAC_1_SQRT_2
        });
        assert_eq!(expected, action);

        tagged = Player::create(1, DEFAULT_PARAMS);
        tagged.position.x += DEFAULT_PARAMS.speed as f32;
        tagged.position.y += DEFAULT_PARAMS.speed as f32;
        untagged = Player::create(2, DEFAULT_PARAMS);
        untagged.position.x += DEFAULT_PARAMS.speed as f32;
        untagged.position.y += DEFAULT_PARAMS.speed as f32;
        let original_dist = tagged.distance(untagged);
        action = untagged.move_towards(tagged, 2000., 2000.);
        match action {
            Tag(_) => {}
            Move(point) => {
                let new_dist = tagged.position.distance(point);
                assert!((original_dist - DEFAULT_PARAMS.speed as f32 - new_dist).abs() < 0.01);
            }
        }
    }

    #[test]
    pub fn move_away() {
        let mut tagged = Player {
            id: 1,
            is_it: true,
            last_tagged: 1,
            position: Point {
                x: 0.0,
                y: 0.0
            },
            speed: 1.0,
            reach: 1.0
        };
        let mut untagged = Player {
            id: 2,
            is_it: true,
            last_tagged: 2,
            position: Point {
                x: 2.0,
                y: 2.0
            },
            speed: 1.0,
            reach: 1.0
        };

        let mut action: Action = untagged.move_away(tagged, 3., 3.);
        let mut expected: Action = Move(Point {
            x: std::f32::consts::FRAC_1_SQRT_2 + 2.,
            y: std::f32::consts::FRAC_1_SQRT_2 + 2.
        });
        assert_eq!(expected, action);
        tagged.is_it = false;
        untagged.is_it = true;
        action = tagged.move_away(untagged, 1000., 600.);
        expected = Move(Point {
            x: 0.,
            y: 0.
        });
        assert_eq!(expected, action);
        tagged.position = Point {
            x: 1.,
            y: 1.
        };
        action = tagged.move_away(untagged, 3., 3.);
        expected = Move(Point {
            x: 1. - std::f32::consts::FRAC_1_SQRT_2,
            y: 1. - std::f32::consts::FRAC_1_SQRT_2
        });
        assert_eq!(expected, action);

        tagged = Player::create(1, DEFAULT_PARAMS);
        tagged.position.x += DEFAULT_PARAMS.speed as f32;
        tagged.position.y += DEFAULT_PARAMS.speed as f32;
        untagged = Player::create(2, DEFAULT_PARAMS);
        untagged.position.x += DEFAULT_PARAMS.speed as f32;
        untagged.position.y += DEFAULT_PARAMS.speed as f32;
        let original_dist = tagged.distance(untagged);
        action = untagged.move_away(tagged, 2000., 2000.);
        match action {
            Tag(_) => {}
            Move(point) => {
                let new_dist = tagged.position.distance(point);
                assert!((original_dist + DEFAULT_PARAMS.speed as f32 - new_dist).abs() < 0.01);
            }
        }

        tagged.position = Point {
            x: 0.,
            y: 0.
        };
        untagged.position = Point {
            x: 0.,
            y: 0.
        };
        action = tagged.move_away(untagged, 1000., 600.);
        expected = Move(Point {
            x: 0.,
            y: 0.
        });
        assert_eq!(expected, action);
    }

}
