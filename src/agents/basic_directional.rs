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
            for player in players {
                if self.player.can_tag(player.player) {
                    let dist = self.player.distance(player.player);
                    if dist <= self.player.reach {
                        // Add the possibility of failed tags, mostly because players get caught in a loop of
                        // tagging each other in clusters otherwise -- TODO move and tag in single turn(?), or dealt with on its own if agents in own threads later.
                        return if thread_rng().gen_bool(0.8) {
                            Action::Tag(player.player.id)
                        } else {
                            log::info!("Tag missed!");
                            self.player.random_move(env.width, env.height)
                        }
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