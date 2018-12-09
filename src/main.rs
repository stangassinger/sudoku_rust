

type SudokuArType = [i8; 81];
//fn placeNumber( pos: i8; sudoku_ar: SudokuArType ){
	
//}


fn solve(sudoku_ar : SudokuArType){
    for elem  in  sudoku_ar.iter() {
	    println!("elem {}",elem);
	}	  	
}


fn main() {
     let sudoku_ar: SudokuArType = [
      8,5,0,0,0,2,4,0,0,
      7,2,0,0,0,0,0,0,9,
      0,0,4,0,0,0,0,0,0,
      0,0,0,1,0,7,0,0,2,
      3,0,5,0,0,0,9,0,0,
      0,4,0,0,0,0,0,0,0,
      0,0,0,0,8,0,0,7,0,
      0,1,7,0,0,0,0,0,0,
      0,0,0,0,3,6,0,4,0
     ];

     solve( sudoku_ar );        
             
}
