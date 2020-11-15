use crate::agents::agent::{Agent, Player};
use crate::action::Action;
use crate::parameters::TagParams;
use crate::tag_environment::TagEnvironment;
use iced_native::Point;
use itertools::Itertools;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalAgent {
    pub player: Player,
}

/// A simple tag strategy to run from 'it' or towards non-'its'.
impl Agent for DirectionalAgent {

    /// If not 'it', run from the nearest 'it'.
    /// If 'it', run to the nearest non-tagback not-'it'.
    fn act(&self, env: &TagEnvironment<Self>) -> Action {
        if self.player.is_it {
            // TODO inefficient here
            let mut sorted = env.agents.iter()
                .filter(|player| !player.player.is_it && player.player.id != self.player.id && player.player.id != self.player.last_tagged)
                .sorted_by(|a, b| self.player.distance(a.player).partial_cmp(&self.player.distance(b.player)).unwrap());
            match sorted.next() {
                None => {
                    // Realistically should fail here.
                    log::warn!("No eligible nearby players found; making random move.");
                    self.player.random_move(env.width, env.height)
                }
                Some(other) => {
                    if self.player.distance(other.player) <= self.player.reach {
                        // Add the possibility of failed tags, mostly because players get caught in a loop of
                        // tagging each other in clusters otherwise -- TODO move and tag in single turn, or dealt with on its own if agents in own threads later.
                        return if thread_rng().gen_bool(0.8) {
                            Action::Tag(other.player.id)
                        } else {
                            log::info!("Tag missed!");
                            self.player.random_move(env.width, env.height)
                        }
                    }
                    log::debug!("Moving towards {:?}", other.player.id);
                    self.player.move_towards(other.player, env.width, env.height)
                }
            }
        } else {
            let it = env.agents.iter().find(|player| player.player.is_it);
            match it {
                None => {
                    log::warn!("No 'it' found; making random move.");
                    self.player.random_move(env.width, env.height)
                }
                Some(it) => {
                    self.player.move_away(it.player, env.width, env.height)
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