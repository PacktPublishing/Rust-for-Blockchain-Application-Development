use near_sdk::{env, near_bindgen, BorshDeserialize, BorshSerialize, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

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
    pub grid: Vec<Vec<Option<char>>>,
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
        let game_id = self.next_game_id;
        let game = CrosswordGame { title, grid };
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
            
            // Check if the word fits
            for &c in &chars {
                if direction == "horizontal" {
                    if current_column >= game.grid[current_row].len() || 
                       (game.grid[current_row][current_column].is_some() && 
                        game.grid[current_row][current_column] != Some(c)) {
                        env::log("Word does not fit in the grid or conflicts with existing letters.".as_bytes());
                        return false;
                    }
                    current_column += 1;
                } else if direction == "vertical" {
                    if current_row >= game.grid.len() || 
                       (game.grid[current_row][current_column].is_some() && 
                        game.grid[current_row][current_column] != Some(c)) {
                        env::log("Word does not fit in the grid or conflicts with existing letters.".as_bytes());
                        return false;
                    }
                    current_row += 1;
                } else {
                    env::log("Invalid direction.".as_bytes());
                    return false;
                }
            }

            // Reset to start position for actual update
            current_row = row;
            current_column = column;

            // Update the grid with the word
            for &c in &chars {
                if direction == "horizontal" {
                    game.grid[current_row][current_column] = Some(c);
                    current_column += 1;
                } else { // direction == "vertical"
                    game.grid[current_row][current_column] = Some(c);
                    current_row += 1;
                }
            }

            // Save the updated game back to storage
            self.games.insert(&game_id, &game);

            env::log(format!("Word '{}' successfully submitted and grid updated for game {}", word, game_id).as_bytes());
            true
        } else {
            env::log(format!("Game {} not found", game_id).as_bytes());
            false
        }
    }
}
