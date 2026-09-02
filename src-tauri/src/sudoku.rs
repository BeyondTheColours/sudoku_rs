//For the purpose of the app, don't need the sudoku to store the possible values.
//Possible values only need to be visible to the user and don't

use core::panic;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use rand::{random_range, rng, seq::SliceRandom};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sudoku(Vec<u8>);

impl Sudoku
//TODO: remove debug trait bound - useful for now for dev.
//Could require display if the intended function of the program is to print the result or write to a file => need a way to generate a text representation of sudoku contents.
{   

    const DIMENSION: u8 = 9;
    const VALUES: [u8; 9] = [1,2,3,4,5,6,7,8,9];


    pub fn from(values: impl IntoIterator<Item = u8>) -> Sudoku{
        values.into_iter().collect()
    }

    pub fn new_zeroed() -> Sudoku{
        let squares: Vec<u8> = (0..(Self::DIMENSION*Self::DIMENSION)).into_iter().map(|_| 0).collect();
        Sudoku(squares)
    }

    pub fn squares(&self) -> &Vec<u8>{
        &self.0
    }

    pub fn gen_solved_new_random() -> Sudoku{
        let mut first_row: Vec<u8> = Self::VALUES.to_vec();
        first_row.shuffle(&mut rng());
        
        //Random permutation of 1..=9
        let first_three_values_of_first_row: Vec<u8> = first_row[0..3].to_vec();
        
        //First three vals share a box with first three vals of the first column => these can't appear again in the box
        //Removing with filter
        let nums_not_in_first_three_of_first_row: Vec<u8> =
            Self::VALUES.iter()
            .filter(|value| !first_three_values_of_first_row.contains(value))
            .map(|value| *value)
            .collect();

        //Using first two of the filtered values as the two values which will be in shared box with the first row
        let second_value_of_first_column: u8 = nums_not_in_first_three_of_first_row[0];
        let third_value_of_first_column: u8 = nums_not_in_first_three_of_first_row[1];

        //Collect together the remaining 6 values of the column which are independent of the first row.
        //Shuffling, otherwise values 2 and 3 of the first row would always be at the end of the first column.
        let mut rest_of_first_column: Vec<u8> =
            vec![
                nums_not_in_first_three_of_first_row[2..].to_vec(),
                first_three_values_of_first_row[1..].to_vec()
            ].concat();
        rest_of_first_column.shuffle(&mut rng());

        //Constructing full first column.
        let first_column: Vec<u8> =
            [vec![first_three_values_of_first_row[0], second_value_of_first_column, third_value_of_first_column],
                rest_of_first_column.to_vec()
            ].concat().to_vec();

        //Adding first element as this is shared between the first column and row

        let mut sudoku: Sudoku = (0..(Self::DIMENSION*Self::DIMENSION))
        .into_iter()
        .map(|i|
            if i < Self::DIMENSION { first_row[i as usize] }
            else if i%Self::DIMENSION == 0 { first_column[(i/Self::DIMENSION) as usize] }
            else { 0 }
        )
        .collect();

        sudoku.solve();

        sudoku

    }

    pub fn puzzle_from_solved_sudoku(&mut self, difficulty: &str) -> Sudoku{
        let amount_to_remove_from_grid: usize;

        match difficulty{
            "easy" => amount_to_remove_from_grid = 2,
            "medium" => amount_to_remove_from_grid = random_range(46..50),
            "hard" => amount_to_remove_from_grid = random_range(51..60),
            _ => panic!("Unknown difficulty: {}", difficulty)
        }

        let mut indices_to_remove: Vec<u8> =
            (0..(Self::DIMENSION*Self::DIMENSION)).into_iter().collect();
            
        indices_to_remove.shuffle(&mut rng());
        indices_to_remove = indices_to_remove[0..amount_to_remove_from_grid].to_vec();

        self.0.iter_mut()
        .enumerate()
        .map(
            |(index, square)|
            if indices_to_remove.contains(&(index as u8)){
                *square = 0;
                square
            } else {
                square
            }
        )
        .map(|square| square.to_owned())
        .collect()

    }

    fn get(&self, x: u8, y: u8) -> Option<&u8>{

        if (x > Self::DIMENSION) || (y > Self::DIMENSION) {
            None
        } else {
            let square_index = (y*Self::DIMENSION) + x;
            self.0.get(square_index as usize)
        }
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut u8>{

        if (x > Self::DIMENSION) || (y > Self::DIMENSION) {
            None
        } else {
            let square_index = (y*Self::DIMENSION) + x;
            self.0.get_mut(square_index as usize)
        }
    }

    fn columns(&self) -> Vec<Vec<&u8>>{
        let dim: u8 = Self::DIMENSION;
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let mut x_offset = 0;

        while x_offset < dim{
            let mut temp: Vec<&u8> = Vec::new();
            let mut y_offset = 0;

            while y_offset < dim{
                let index: u8 = (dim*y_offset) + x_offset;
                let square: &u8 = self.0.get(index as usize).unwrap();
                temp.push(square);
                y_offset += 1;
            }
            res.push(temp);
            x_offset += 1;

        }
        res
    }

    fn rows(&self) -> Vec<Vec<&u8>>{
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let squares = &self.0;
        let dim = Self::DIMENSION;

        let mut x_count = 0;
        let mut y_count = 0;

        while y_count < dim{
            let mut temp: Vec<&u8> = Vec::new();

            while x_count < dim{
                let index = (y_count*dim) + x_count;
                let square = squares.get(index as usize).unwrap();
                temp.push(square);
                x_count += 1;
            }
            res.push(temp);
            y_count += 1;
            x_count = 0;
        }
        
        res
    }
    
    fn boxes(&self) -> Vec<Vec<&u8>>{
        let mut res: Vec<Vec<&u8>> = Vec::new();

        let mut x_count = 0;
        let mut y_count = 0;
        let dim = Self::DIMENSION;
        let sqrt = dim.isqrt();
        let squares = &self.0;

        while y_count < dim{
            while x_count < dim{
                let mut temp: Vec<&u8> = Vec::new();
                let mut inner_x = x_count;
                let mut inner_y = y_count;

                while inner_y < (y_count + sqrt){
                    while inner_x < (x_count + sqrt){
                        let index = (inner_y*dim) + inner_x;
                        let square = squares.get(index as usize).unwrap();
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

        let dim = Self::DIMENSION;

        if x > dim{
            panic!("x coordinate outside possible bounds.\nx coordinate: {}\ncoordinate range 1-{}", x, dim)

        } else if y > dim{
            panic!("y coordinate outside possible bounds.\ny coordinate: {}\ncoordinate range 1-{}", y, dim)

        } else {
            let sqrt: u8 = dim.isqrt();
            ((x/sqrt), y/sqrt)
        }

    }

    fn column_except(&self, x: u8, y: u8) -> Vec<&u8>{
        let mut res: Vec<&u8> = Vec::new();

        for inner_y in 0..Self::DIMENSION{
            if inner_y != y{
                let index = inner_y*Self::DIMENSION + x;
                let square = self.0.get(index as usize).unwrap();
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
                let square = self.0.get(index as usize).unwrap();
                res.push(square);
            }
        }
        res
    }

    fn box_except(&self, x: u8, y: u8) -> Vec<&u8> {

        let sqrt: u8 = Self::DIMENSION.isqrt();
        let square_not_included_index = (y*Self::DIMENSION) + x;

        let (box_x, box_y) = self.box_containing(x, y);
        let (start_x, start_y) = (box_x*sqrt, box_y*sqrt);

        let mut res: Vec<&u8> = Vec::new();

        for i in start_y..(start_y+sqrt){
            for j in start_x..(start_x+sqrt){
                if (i*Self::DIMENSION)+j != square_not_included_index{
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
        .filter(
            |value| !subset.iter().any(|square| **square == **value)
        )
        .map(|value| *value)
        .collect()
    }

    fn subset_is_solved(squares: &Vec<&u8>) -> bool{
        Self::VALUES
        .iter()
        .all(
            |value|
            squares
            .iter()
            .filter(|square| ***square == *value)
            .collect::<Vec<&&u8>>()
            .len() == 1
        )
    }

    pub fn is_solved(&self) -> bool{
        /*
        for column in self.columns(){
            if !Self::subset_is_solved(column){
                return false
            }
        }

        for row in self.rows(){
            if !Self::subset_is_solved(row){
                return false
            }
        }

        for box_ in self.boxes(){
            if !Self::subset_is_solved(box_){
                return false
            }
        }
        true
        */

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
            .filter(
                |value|
                values_not_in_box.contains(value) &&
                values_not_in_row.contains(value) &&
                values_not_in_column.contains(value)
            )
            .map(|value| *value)
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
                sudoku.0[index] = value;

                if inner(sudoku, index+1){
                    return true
                } else {
                    sudoku.0[index] = 0;
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

        for square in self.0.iter(){
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
    //Useful to create a sudoku from an iterator of Vec<u8>
    //e.g. after mapping and such
    //Panics if the number of elements in the iterator != 81.
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {

        let squares: Vec<u8> = iter
            .into_iter()
            .collect::<Vec<u8>>();

        if squares.len() != (Self::DIMENSION*Self::DIMENSION) as usize{
            //Using panic if the wrong numbers of elements is present.
            //This is not somthing the user can control
            panic!("Cannot construct a Sudoku from collect with {} elements\nExactly {} elements required for Sudoku due to 9x9 Self::DIMENSION", squares.len(), Self::DIMENSION*Self::DIMENSION)
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
        for i in 0..sudoku.0.len(){
            if sudoku.0[i] != 0 {
                let (x, y) = (i as u8%Sudoku::DIMENSION, i as u8/Sudoku::DIMENSION);
                let square: &u8 = &sudoku.0[i];
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

        let puzzle: Sudoku = Sudoku::from(PUZZLE);

        let solution: Sudoku = Sudoku::from(SOLUTION);

        let incorrect_solution = Sudoku::from(INCORRECT_SOLTUION);
        
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
        
        let mut sudoku: Sudoku = Sudoku::from(PUZZLE);

        let solution: Sudoku = Sudoku::from(SOLUTION);

        sudoku.solve();

        assert!(sudoku.is_solved());
        assert!(solution.is_solved());
        assert_eq!(solution, sudoku);
    }

    #[test]
    fn values_not_in_subet(){
        let sudoku: Sudoku = Sudoku::from(PUZZLE);

        let (x, y) = (1, 0);

        let values_not_in_box: Vec<u8> = Sudoku::values_not_in_subet_singles(sudoku.box_except(x, y));

        assert_eq!(values_not_in_box, vec![1,4,6,8,9]);
    }

    #[test]
    fn possibles(){

        let sudoku = Sudoku::from(PUZZLE);

        let (x, y) = (2, 1);
        let index = x + (y*Sudoku::DIMENSION);

        let possible_values: Vec<u8> = sudoku.possible_values(index.into());

        let expected_res: Vec<u8> = vec![9];
        assert_eq!(expected_res, possible_values);
    }
}