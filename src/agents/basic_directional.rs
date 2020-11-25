use crate::agents::agent::{Agent, Player};
use crate::action::Action;
use crate::parameters::TagParams;
use crate::tag_environment::TagEnvironment;
use iced_native::Point;
use rand::{thread_rng, Rng};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalAgent {
    pub player: Player,
}

/// A simple tag strategy to run from 'it's or towards non-'it's.
impl Agent for DirectionalAgent {

    /// If not 'it', run from the nearest 'it'.
    /// If 'it', run to the nearest non-tagback not-'it'.
    fn act(&self, env: &TagEnvironment<Self>) -> Action {
        if self.player.is_it {
            let mut players = env.agents.iter();
            let mut nearest = players.next().unwrap();
            let mut nearest_distance = if self.player.can_tag(nearest.player) { nearest.player.distance(self.player) } else { f32::MAX };
            if nearest_distance <= self.player.reach {
                return self.tag(*nearest, env);
            }
            for player in players {
                if self.player.can_tag(player.player) {
                    let dist = self.player.distance(player.player);
                    if dist <= self.player.reach {
                        return self.tag(*player, env);
                    }
                    if dist < nearest_distance {
                        nearest_distance = dist;
                        nearest = player;
                    }
                }
            }
            log::debug!("Moving towards {:?}", nearest.player.id);
            self.player.move_towards(nearest.player, env.width, env.height)
        } else {
            let mut nearest_it = env.it.iter()
                .filter(|player| env.agents.get(player).unwrap().player.last_tagged != self.player.id)
                .sorted_by(|a, b| {
                    self.player.distance(env.agents.get(a).unwrap().player)
                        .partial_cmp(&self.player.distance(env.agents.get(b).unwrap().player)).unwrap()
                });
            match nearest_it.next() {
                None => {
                    log::warn!("No 'it' found; making random move.");
                    self.player.random_move(env.width, env.height)
                }
                Some(it) => {
                    self.player.move_away(env.agents.get(it).unwrap().player, env.width, env.height)
                }
            }
        }
    }

    fn create(id: usize, params: TagParams) -> Self {
        DirectionalAgent {
            player: Player::create(id, params),
        }
    }

    fn update(&mut self, position: &Point) {
        self.player.update(position)
    }

    fn player(&self) -> Player {
        self.player
    }

    fn tag(&mut self, by: usize) {
        self.player.tag(by)
    }

    fn untag(&mut self) {
        self.player.untag()
    }

}

impl DirectionalAgent {

    fn tag(&self, other: Self, env: &TagEnvironment<Self>) -> Action {
        // Add the possibility of failed tags, mostly because players get caught in a loop of
        // tagging each other in clusters otherwise -- TODO move and tag in single turn(?), or dealt with on its own if agents in own threads later.
        return if thread_rng().gen_bool(0.8) {
            Action::Tag(other.player.id)
        } else {
            log::info!("Tag missed!");
            self.player.random_move(env.width, env.height)
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::tag_environment::TagEnvironment;
    use crate::agents::basic_directional::DirectionalAgent;
    use dashmap::DashMap;
    use std::collections::HashSet;
    use crate::agents::agent::{Player, Agent};
    use iced::Point;
    use crate::environment::Environment;
    use crate::action::Action::Tag;

    #[test]
    fn tag_zero() {
        let env = base_env();
        let player = env.agents.get(&1).unwrap();
        let mut passed = false;
        for _ in 0..10 {
            let action = player.act(&env);
            if action.eq(&Tag(0)) {
                passed = true;
                break;
            }
        }
        assert!(passed);
    }

    fn base_env() -> TagEnvironment<DirectionalAgent> {
        let mut env: TagEnvironment<DirectionalAgent> = TagEnvironment {
            agents: DashMap::with_capacity(2),
            width: 2.,
            height: 2.,
            it: HashSet::new(),
            show_numbers: false
        };
        let agent0: DirectionalAgent = DirectionalAgent {
            player: Player {
                id: 0,
                is_it: false,
                last_tagged: 0,
                position: Point {
                    x: 0.0,
                    y: 0.0,
                },
                speed: 2.0,
                reach: 2.0
            }
        };
        let agent1: DirectionalAgent = DirectionalAgent {
            player: Player {
                id: 1,
                is_it: true,
                last_tagged: 1,
                position: Point {
                    x: 0.5,
                    y: 0.5,
                },
                speed: 2.0,
                reach: 2.0
            }
        };
        env.it.insert(1);
        env.add_agent( agent0);
        env.add_agent( agent1);
        env
    }

}