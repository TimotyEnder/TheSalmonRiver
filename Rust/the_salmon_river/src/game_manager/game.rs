use godot::prelude::*;
pub struct Game {
    winning_player_number: Option<u8>,
    rounds_to_win: usize,
    rounds_by_player_number: Vec<usize>,
    current_round: usize,
    round_time_in_sec: usize,
}
impl Game {
    pub fn new(number_of_players: u8, rounds_to_win: usize, round_time_in_sec: usize) -> Self {
        Self {
            winning_player_number: None,
            rounds_to_win,
            rounds_by_player_number: vec![0; number_of_players as usize],
            current_round: 0,
            round_time_in_sec: round_time_in_sec,
        }
    }
    pub fn get_player_number(&self) -> usize {
        self.rounds_by_player_number.len()
    }
    pub fn get_winner(&self) -> Option<u8> {
        self.winning_player_number
    }
    pub fn log_player_win(&mut self, player_num: u8) {
        self.rounds_by_player_number[player_num as usize] += 1;
        if self.rounds_by_player_number[player_num as usize] >= self.rounds_to_win {
            self.winning_player_number = Some(player_num);
        }
        self.current_round += 1;
    }
    pub fn log_tie(&mut self) {
        self.current_round += 1;
    }
    pub fn get_current_round(&self) -> u32 {
        self.current_round as u32
    }
    pub fn get_score_for_player(&self, player_num: u8) -> u32 {
        self.rounds_by_player_number
            .get(player_num as usize)
            .copied()
            .unwrap_or(0) as u32
    }
    pub fn get_round_time(&self) -> u32 {
        self.round_time_in_sec as u32
    }
}
