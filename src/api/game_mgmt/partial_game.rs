use crate::tafl::game::PlayerInfo;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PartialGame {
    pub(crate) game_id: usize,
    pub(crate) player: PlayerInfo,
    pub(crate) time_start: usize,
    pub(crate) time_incr: usize,
    pub(crate) created_at: usize,
}