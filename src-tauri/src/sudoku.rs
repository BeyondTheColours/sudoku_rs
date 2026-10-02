use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use rand::{random_range, rng, seq::SliceRandom};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sudoku(Vec<u8>);

impl Sudoku {
    //Side length of a 9x9 sudoku. Using a const so there aren't magic number 9s everywhere   
    const DIMENSION: u8 = 9;
    //Alias for the values 1..=9
    const VALUES: [u8; 9] = [1,2,3,4,5,6,7,8,9];

    //
    fn new_zeroed() -> Sudoku{
        (0..(Self::DIMENSION*Self::DIMENSION))
        .map(|_| 0)
        .collect()
    }

    //Return ref to underlying data of the sudoku
    fn squares(&self) -> &Vec<u8>{
        &self.0
    }

    //Modify the value at a given index in place
    //Panic is index is out of bounds
    fn replace(&mut self, index: usize, value: u8) -> (){
        if index >= (Self::DIMENSION*Self::DIMENSION) as usize{
            let max_index = (Self::DIMENSION*Self::DIMENSION) -1;
            panic!("Called \'replace\' with index {} but maximum allowed index is {}", index, max_index);
        }
        //Can't use .squares() as this provides only an immutable reference
        self.0[index] = value;
    }

    pub fn gen_solved_new_random() -> Sudoku{
        //Initialise with all 0's
        let mut sudoku = Sudoku::new_zeroed();
        //First row is a random shuffle of VALUES. All permutations are allowed
        let mut first_row: Vec<u8> = Self::VALUES.to_vec();
        first_row.shuffle(&mut rng());

        //Adding first row to sudoku
        for (index, value) in first_row.iter().enumerate(){
            sudoku.replace(index, *value);
        }

        //Generating random first column using .possible_values() ot ensure only
        //values leading to a solvable puzzle are selected
        for i in 1..(Self::DIMENSION as usize){
            //Index of the first square of each row beginning from the second row
            //First row is already filled
            let index: usize = 9*i;
            let possible_values: Vec<u8> = sudoku.possible_values(index);
            
            //Select random index to take one of the possible_values
            //possible_values is sorted into numerical order, so need to pick a random value
            //so the resultant puzzles are actually random
            let random_index: usize = random_range(0..possible_values.len());
            let value: u8 = possible_values[random_index];
            sudoku.replace(index, value);
        }
        
        //Filling first row and column gives enough information to solve the whole thing
        sudoku.solve();
        sudoku

    }

    pub fn puzzle_from_solved_sudoku(&mut self, difficulty: &str) -> Sudoku{
        let amount_to_remove_sudoku_from_grid: usize;

        //Removing values from the solved sudoku to generate the puzzle
        match difficulty{
            "easy" => amount_to_remove_sudoku_from_grid = random_range(31..40),
            "medium" => amount_to_remove_sudoku_from_grid = random_range(41..50),
            "hard" => amount_to_remove_sudoku_from_grid = random_range(51..60),
            _ => panic!("Unknown difficulty: {}", difficulty)
        }

        //Generating random set of indices to be removed based on the difficult
        let mut all_indices: Vec<u8> =
            (0..(Self::DIMENSION*Self::DIMENSION))
            .collect();
            
        all_indices.shuffle(&mut rng());
        let indices_to_remove: Vec<u8> = all_indices[0..amount_to_remove_sudoku_from_grid].to_vec();

        //Iterate through the whole sudoku and set the values of at indices_to_remove to 0
        //No dedicated empty state for the squares to 0 is used instead
        //Considered using Option<u8> to have an empty state (i.e. None) for the squares
        //but found there wasn't much benefit compared to just using u8's
        self.0.iter_mut()
        .enumerate()
        .map(
            |(index, square)|
            if indices_to_remove.contains(&(index as u8)){
                *square = 0;
                square.to_owned()
            } else {
                square.to_owned()
            }
        )
        .collect()

    }

    //Retrieve the value of a given square based on x,y coordinates in the grid.
    fn get(&self, x: u8, y: u8) -> Option<&u8>{

        if (x > Self::DIMENSION) || (y > Self::DIMENSION) {
            None
        } else {
            let square_index = (y*Self::DIMENSION) + x;
            self.squares().get(square_index as usize)
        }
    }

    //Returns a list of the 9 columns making up the sudoku.
    //top to bottom
    fn columns(&self) -> Vec<Vec<&u8>>{
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let mut x_offset: u8 = 0;

        //Outer loop incrementing x coord
        while x_offset < Self::DIMENSION{
            let mut temp: Vec<&u8> = Vec::new();
            let mut y_offset = 0;

            //Inner loop incrementing y coord
            while y_offset < Self::DIMENSION{
                let index: u8 = (Self::DIMENSION*y_offset) + x_offset;
                let square: &u8 = self.squares().get(index as usize).unwrap();
                temp.push(square);
                y_offset += 1;
            }
            res.push(temp);
            x_offset += 1;
        }
        res
    }

    //Returns a list of the 9 rows making up the sudoku.
    //top to bottom
    fn rows(&self) -> Vec<Vec<&u8>>{
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let mut x_count: u8 = 0;
        let mut y_count: u8 = 0;

        //Outer loop incrementing y coord
        while y_count < Self::DIMENSION{
            let mut temp: Vec<&u8> = Vec::new();

            //Inner loop incrementing x coord
            while x_count < Self::DIMENSION{
                let index: u8 = (y_count*Self::DIMENSION) + x_count;
                let square: &u8 = self.squares().get(index as usize).unwrap();
                temp.push(square);
                x_count += 1;
            }
            res.push(temp);
            y_count += 1;
            x_count = 0;
        }
        
        res
    }
    
    //Returns a list of the 9 3x3 boxes making up the sudoku.
    //left to right, top to bottom
    fn boxes(&self) -> Vec<Vec<&u8>>{
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let mut x_count: u8 = 0;
        let mut y_count: u8 = 0;
        let sqrt: u8 = Self::DIMENSION.isqrt();

        while y_count < Self::DIMENSION{
            while x_count < Self::DIMENSION{
                let mut temp: Vec<&u8> = Vec::new();
                let mut inner_x: u8 = x_count;
                let mut inner_y: u8 = y_count;

                
                while inner_y < (y_count + sqrt){
                    while inner_x < (x_count + sqrt){
                        let index: u8 = (inner_y*Self::DIMENSION) + inner_x;
                        let square: &u8 = self.squares().get(index as usize).unwrap();
                        temp.push(square);
                        inner_x += 1;
                    }
                    inner_y += 1;
                    inner_x = x_count;
                }
                x_count += sqrt;
                res.push(temp);
            }
            y_count += sqrt;
            x_count = 0;
        }
        res
    }

    fn box_containing(&self, x: u8, y: u8) -> (u8, u8) {

        if x > Self::DIMENSION{
            panic!("x coordinate outside possible bounds.\nx coordinate: {}\ncoordinate range 1-{}", x, Self::DIMENSION)

        } else if y > Self::DIMENSION{
            panic!("y coordinate outside possible bounds.\ny coordinate: {}\ncoordinate range 1-{}", y, Self::DIMENSION)

        } else {
            let sqrt: u8 = Self::DIMENSION.isqrt();
            ((x/sqrt), y/sqrt)
        }

    }

    fn column_except(&self, x: u8, y: u8) -> Vec<&u8>{
        let mut res: Vec<&u8> = Vec::new();

        for inner_y in 0..Self::DIMENSION{
            if inner_y != y{
                let index: u8 = inner_y*Self::DIMENSION + x;
                let square: &u8 = self.squares().get(index as usize).unwrap();
                res.push(square)
            }
        }
        res
    }

    fn row_except(&self, x: u8, y: u8) -> Vec<&u8> {
        let mut res: Vec<&u8> = Vec::new();

        for inner_x in 0..Self::DIMENSION{
            if inner_x != x{
                let index = y*Self::DIMENSION + inner_x;
                let square = self.squares().get(index as usize).unwrap();
                res.push(square);
            }
        }
        res
    }

    fn box_except(&self, x: u8, y: u8) -> Vec<&u8> {

        let sqrt: u8 = Self::DIMENSION.isqrt();
        let excluded_square_index: u8 = (y*Self::DIMENSION) + x;

        let (box_x, box_y) = self.box_containing(x, y);
        let (start_x, start_y) = (box_x*sqrt, box_y*sqrt);

        let mut res: Vec<&u8> = Vec::new();

        for i in start_y..(start_y+sqrt){
            for j in start_x..(start_x+sqrt){
                if (i*Self::DIMENSION)+j != excluded_square_index{
                    res.push(self.get(j, i).unwrap())
                }
            }
        }
        res
    }

    //For the given subset, return any of Self::VALUES (1..=9) not present in that subset in squares with only a single value.
    //Does not look at the values contained in multiples - i.e. squares such as Option<u8>([1,3..]).
    fn values_not_in_subet_singles(subset: Vec<&u8>) -> Vec<u8> {
        Self::VALUES.iter()
        .filter_map(
            |value|
            if subset.iter().any(|square| **square == *value){ None }
            else { Some(*value) }
        )
        .collect()
    }

    //squares is a row, column or box
    //Checks if each number (1..=9) appears exactly once in the given subset
    //Return false otherwise
    fn subset_is_solved(squares: &Vec<&u8>) -> bool{
        Self::VALUES
        .iter()
        .all(
            |value|
            squares
            .iter()
            .filter(|square| ***square == *value)
            .count() == 1
        )
    }

    pub fn is_solved(&self) -> bool{

        self.columns().iter()
        .all(|column| Self::subset_is_solved(column))&&
        
        self.rows().iter()
        .all(|row|  Self::subset_is_solved(row))&&

        self.boxes().iter()
        .all(|box_| Self::subset_is_solved(box_))
        
    }


    fn possible_values(&self, index: usize) -> Vec<u8>{
        let (x, y) = (index as u8%Self::DIMENSION, index as u8/Self::DIMENSION);
        
        let values_not_in_box: Vec<u8> = Self::values_not_in_subet_singles(self.box_except(x, y));
        let values_not_in_row: Vec<u8> = Self::values_not_in_subet_singles(self.row_except(x, y));
        let values_not_in_column: Vec<u8> = Self::values_not_in_subet_singles(self.column_except(x, y));

        let possible_values: Vec<u8> =
            Self::VALUES
            .iter()
            .filter_map(
                |value|
                
                if values_not_in_box.contains(value) && values_not_in_row.contains(value) && values_not_in_column.contains(value) {
                    Some(*value)
                } else {
                    None
                }
            )
            .collect();
        
        possible_values
    }

    pub fn solve(&mut self) -> bool{

        fn inner(sudoku: &mut Sudoku, index: usize) -> bool {

            if index == ((Sudoku::DIMENSION)*(Sudoku::DIMENSION)) as usize{
                return true
            }

            if sudoku.squares()[index] != 0{
                return inner(sudoku, index+1)
            }

            let possible_values: Vec<u8> = sudoku.possible_values(index);
            
            for value in possible_values{
                sudoku.replace(index, value);

                if inner(sudoku, index+1){
                    return true
                } else {
                    sudoku.replace(index, 0);
                }
            }
            false
        }
        inner(self, 0)
    }
}

impl std::fmt::Display for Sudoku
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::new();
        let dim: usize = Self::DIMENSION as usize;
        let mut count: usize = 1;

        for square in self.squares(){
            let mut temp: String = String::new();
            temp.push_str(format!("{:?}", square).as_str());
            if count == (dim){
                temp.push('\n');
                count = 0;
            }
            res.push_str(temp.as_str());
            count += 1;
        }
        write!(f, "{}", res)
    }
}

impl<'a> IntoIterator for &'a Sudoku{
    type Item = &'a u8;

    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.squares().into_iter()
    }
}

impl FromIterator<u8> for Sudoku{
    //Useful to create a sudoku sudoku_from an iterator of Vec<u8>
    //e.g. after mapping and such
    //Panics if the number of elements in the iterator != 81.
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {

        let squares: Vec<u8> = iter
            .into_iter()
            .collect::<Vec<u8>>();

        if squares.len() != (Self::DIMENSION*Self::DIMENSION) as usize{
            //Using panic if the wrong numbers of elements is present.
            //This is not somthing the user can control
            panic!("Cannot construct a Sudoku sudoku_from collect with {} elements\nExactly {} elements required for Sudoku due to 9x9 Self::DIMENSION", squares.len(), Self::DIMENSION*Self::DIMENSION)
        }
        
        Sudoku(squares)
    }
}

#[cfg(test)]
mod test{

use super::*;

    //Function used for testing "gen_new_random".
    //Need to ensure that puzzle it produces is indeed solvable
    fn is_solvable(sudoku: &Sudoku) -> bool{
        for i in 0..sudoku.squares().len(){
            if sudoku.squares()[i] != 0 {
                let (x, y) = (i as u8%Sudoku::DIMENSION, i as u8/Sudoku::DIMENSION);
                let square: &u8 = &sudoku.squares()[i];
                let groups: Vec<Vec<&u8>> = vec![sudoku.box_except(x, y), sudoku.row_except(x, y), sudoku.column_except(x, y)];
                let square_in_any_group: bool =
                groups.iter()
                .any(|group| group.contains(&square));
                if square_in_any_group{
                    return false
                }
            }
        }
        true
    }

    //Helper to convert impl IntoIterator into a Sudoku
    fn sudoku_from(values: impl IntoIterator<Item = u8>) -> Sudoku{
        //FromIterator implemented for Sudoku
        values.into_iter().collect()
    }

    //Puzzle which is definitely solvable
    const PUZZLE: [u8; (Sudoku::DIMENSION*Sudoku::DIMENSION) as usize] =
        [
            3,0,0,0,4,0,0,0,0,
            0,0,0,6,0,0,5,0,1,
            7,5,2,0,0,1,0,0,0,
            0,0,1,0,0,0,7,0,0,
            5,0,0,3,9,6,0,0,0,
            0,0,8,1,5,0,0,9,6,
            0,0,3,0,1,0,0,6,0,
            0,0,4,0,0,0,1,0,0,
            0,0,0,0,2,8,0,0,0
        ];
    
    //Solution to puzzle
    const SOLUTION: [u8; 81] =
        [
            3,1,6,5,4,9,8,2,7,
            4,8,9,6,7,2,5,3,1,
            7,5,2,8,3,1,6,4,9,
            6,9,1,2,8,4,7,5,3,
            5,4,7,3,9,6,2,1,8,
            2,3,8,1,5,7,4,9,6,
            8,7,3,4,1,5,9,6,2,
            9,2,4,7,6,3,1,8,5,
            1,6,5,9,2,8,3,7,4,
        ];
    
    //Incorrect solution to PUZZLE used to verify an incorrect solution can be detected
    const INCORRECT_SOLTUION: [u8; 81] =
        [
            3,1,6,5,4,9,8,2,7,
            4,8,9,6,7,2,5,3,1,
            7,5,2,8,3,1,6,4,9,
            6,9,1,2,8,4,7,5,3,
            5,4,7,3,9,6,2,1,8,
            2,3,8,1,5,7,4,9,6,
            8,7,3,4,1,1,9,6,2,
            9,2,4,7,6,3,1,8,5,
            1,6,5,9,2,8,3,7,4,
        ];

    #[test]
    fn is_solved(){

        let puzzle: Sudoku = PUZZLE.into_iter().collect();

        let solution: Sudoku = SOLUTION.into_iter().collect();

        let incorrect_solution: Sudoku = INCORRECT_SOLTUION.into_iter().collect();
        
        assert!(!(puzzle.is_solved()));
        assert!(solution.is_solved());
        assert!(!(incorrect_solution.is_solved()));
    }

    #[test]
    fn new_random_puzzle(){

        for _ in 0..100{
            //Generate 100 random solved sudokus
            let mut solution = Sudoku::gen_solved_new_random();
            //Test that each is solved
            assert!(solution.is_solved());
            
            //Generate a puzzle of "hard" difficulty
            let mut puzzle = solution.puzzle_from_solved_sudoku("hard");
            //Test that this puzzle is technically solvable
            assert!(is_solvable(&puzzle));
            //Solve puzzle
            puzzle.solve();
            //Test the solution is correct
            assert!(puzzle.is_solved());
        }
    }

    #[test]
    fn solve(){
        
        let mut sudoku: Sudoku = sudoku_from(PUZZLE);

        let solution: Sudoku = sudoku_from(SOLUTION);

        sudoku.solve();

        assert!(sudoku.is_solved());
        assert!(solution.is_solved());
        assert_eq!(solution, sudoku);
    }

    #[test]
    fn values_not_in_subet(){
        let sudoku: Sudoku = sudoku_from(PUZZLE);

        let (x, y) = (1, 0);

        let values_not_in_box: Vec<u8> = Sudoku::values_not_in_subet_singles(sudoku.box_except(x, y));

        assert_eq!(values_not_in_box, vec![1,4,6,8,9]);
    }

    #[test]
    fn possibles(){

        let sudoku = sudoku_from(PUZZLE);

        let (x, y) = (2, 1);
        let index = x + (y*Sudoku::DIMENSION);

        let possible_values: Vec<u8> = sudoku.possible_values(index.into());

        let expected_res: Vec<u8> = vec![9];
        assert_eq!(expected_res, possible_values);
    }
}