use crate::bitboards::BitBoards;

pub struct BoardState {
    position: BitBoards,
    pub turn_count: u16,
    pub fifty_move_rule: u8,
    pub can_castle_kingside_white: bool,
    pub can_castle_kingside_black: bool,
    pub can_castle_queenside_white: bool,
    pub can_castle_queenside_black: bool,
    history: Vec<BoardState>,
}

impl BoardState {
    pub fn get_position(&self) -> &BitBoards {
        &self.position
    }

    pub fn get_mut_position(&mut self) -> &mut BitBoards {
        &mut self.position
    }

    pub fn get_turn(&self, index: usize) -> Option<&Self> {
        if index == 0 {
            return None;
        }

        self.history.get(index - 1)
    }
}
