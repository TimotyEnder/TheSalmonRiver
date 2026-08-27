pub struct Game {
    winning_team_number: Option<u8>,
    rounds_to_win: usize,
    rounds_by_team: Vec<usize>,
    current_round: usize,
    round_time_in_sec: usize,
    teams: Vec<Vec<u8>>,
    player_num: u8,
}
impl Game {
    pub fn new(number_of_players: u8, rounds_to_win: usize, round_time_in_sec: usize) -> Self {
        Self {
            winning_team_number: None,
            rounds_to_win,
            rounds_by_team: vec![0; 4 as usize],
            current_round: 0,
            round_time_in_sec: round_time_in_sec,
            teams: vec![vec![1], vec![2], vec![3], vec![4]],
            player_num: number_of_players,
        }
    }
    pub fn set_player_team(&mut self, player_num: u8, team_num: u8) {
        for team in self.teams.iter_mut() {
            let posible_position_to_remove = team.iter().position(|element| *element == player_num);
            if let Some(pos) = posible_position_to_remove {
                team.remove(pos);
            }
        }
        self.teams[team_num as usize - 1].push(player_num);
    }
    pub fn get_player_team(&self, player_num: u8) -> Option<u8> {
        let mut index: u8 = 0;
        for team in self.teams.iter() {
            if team.contains(&player_num) {
                return Some(index + 1);
            }
            index += 1;
        }
        return None;
    }
    pub fn get_player_number(&self) -> usize {
        self.player_num as usize
    }
    pub fn get_team_number(&self) -> usize {
        let mut count = 0;
        for team in self.teams.iter() {
            if !team.is_empty() {
                count += 1;
            }
        }
        count
    }
    pub fn get_winning_team(&self) -> Option<u8> {
        self.winning_team_number
    }
    pub fn log_team_win(&mut self, team_num: u8) {
        self.rounds_by_team[team_num as usize - 1] += 1;
        if self.rounds_by_team[team_num as usize - 1] >= self.rounds_to_win {
            self.winning_team_number = Some(team_num);
        }
        self.current_round += 1;
    }
    pub fn log_tie(&mut self) {
        self.current_round += 1;
    }
    pub fn get_current_round(&self) -> u32 {
        self.current_round as u32
    }
    pub fn get_score_for_team(&self, team_num: u8) -> u32 {
        self.rounds_by_team
            .get(team_num as usize - 1)
            .copied()
            .unwrap_or(0) as u32
    }
    pub fn get_round_time(&self) -> u32 {
        self.round_time_in_sec as u32
    }
    pub fn get_rounds_to_win(&self) -> u32 {
        self.rounds_to_win as u32
    }
}
