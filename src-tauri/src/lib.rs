// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

//TODO enum for the different difficulties instead of using strings
pub mod sudoku;
use sudoku::Sudoku;

#[tauri::command]
fn gen_random_sudoku(difficulty: &str) -> Sudoku {
    let mut sudoku: Sudoku = Sudoku::gen_solved_new_random();
    //println!("{}\n{}", difficulty, sudoku);
    sudoku.puzzle_from_solved_sudoku(difficulty)
}

#[tauri::command]
fn is_solved(sudoku: sudoku::Sudoku) -> bool {
    sudoku.is_solved()
}

#[tauri::command]
fn solve( mut sudoku: sudoku::Sudoku ) -> sudoku::Sudoku{
    //println!("input:\n{}\n", sudoku);
    sudoku.solve();
    //println!("solution:\n{}", sudoku);
    sudoku
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![gen_random_sudoku, is_solved, solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}