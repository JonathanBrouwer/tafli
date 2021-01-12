#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PreviousMoveInfo {
    pub last_move: Option<((usize, usize), (usize, usize))>,
    pub captures: Vec<(usize, usize)>,
}

impl PreviousMoveInfo {
    pub(crate) fn new() -> Self {
        PreviousMoveInfo {
            last_move: None,
            captures: Vec::new(),
        }
    }
}