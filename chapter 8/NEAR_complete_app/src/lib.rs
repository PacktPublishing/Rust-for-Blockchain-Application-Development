use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CrosswordGameContract {
    games: UnorderedMap<u64, CrosswordGame>,
    next_game_id: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CrosswordGame {
    pub title: String,
    pub grid: Vec<Vec<Option<String>>>, // Storing each cell as a String
}

#[near_bindgen]
impl CrosswordGameContract {
    #[init]
    pub fn new() -> Self {
        Self {
            games: UnorderedMap::new(b"g"),
            next_game_id: 0,
        }
    }

    pub fn create_game(&mut self, title: String, grid: Vec<Vec<Option<char>>>) -> u64 {
        let grid_string = grid.into_iter()
            .map(|row| row.into_iter()
                .map(|opt_char| opt_char.map(|c| c.to_string()))
                .collect())
            .collect();

        let game_id = self.next_game_id;
        let game = CrosswordGame { title, grid: grid_string };
        self.games.insert(&game_id, &game);
        self.next_game_id += 1;
        game_id
    }

    pub fn get_game(&self, game_id: u64) -> Option<CrosswordGame> {
        self.games.get(&game_id)
    }

    pub fn submit_word(&mut self, game_id: u64, word: String, row: usize, column: usize, direction: String) -> bool {
        if let Some(mut game) = self.games.get(&game_id) {
            let chars: Vec<char> = word.chars().collect();
            let mut current_row = row;
            let mut current_column = column;
            
            for &c in &chars {
                let c_string = c.to_string();
                if direction == "horizontal" {
                    if current_column >= game.grid[current_row].len() || 
                       (game.grid[current_row][current_column].is_some() && 
                        game.grid[current_row][current_column] != Some(c_string.clone())) {
                        env::log_str("Word does not fit in the grid or conflicts with existing letters.");
                        return false;
                    }
                    current_column += 1;
                } else if direction == "vertical" {
                    if current_row >= game.grid.len() || 
                       (game.grid[current_row][current_column].is_some() && 
                        game.grid[current_row][current_column] != Some(c_string.clone())) {
                        env::log_str("Word does not fit in the grid or conflicts with existing letters.");
                        return false;
                    }
                    current_row += 1;
                } else {
                    env::log_str("Invalid direction.");
                    return false;
                }
            }

            current_row = row;
            current_column = column;

            for &c in &chars {
                let c_string = c.to_string();
                if direction == "horizontal" {
                    game.grid[current_row][current_column] = Some(c_string);
                    current_column += 1;
                } else { // direction == "vertical"
                    game.grid[current_row][current_column] = Some(c_string);
                    current_row += 1;
                }
            }

            self.games.insert(&game_id, &game);

            env::log_str(&format!("Word '{}' successfully submitted and grid updated for game {}", word, game_id));
            return true;
        } else {
            env::log_str(&format!("Game {} not found", game_id));
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_game() {
        let mut contract = CrosswordGameContract::new();
        let game_id = contract.create_game("Test Game".to_string(), vec![vec![Some('A'), None], vec![None, Some('B')]]);
        
        assert_eq!(game_id, 0);
    }

    #[test]
    fn test_get_game() {
        let mut contract = CrosswordGameContract::new();
        contract.create_game("Test Game".to_string(), vec![vec![Some('A'), None], vec![None, Some('B')]]);
        
        let game = contract.get_game(0).unwrap();
        assert_eq!(game.title, "Test Game");
    }

}
