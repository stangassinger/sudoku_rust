

type SudokuArType = [i8; 81];


fn place_number( pos: i8, sudoku_ar: SudokuArType ) -> bool {
	let mut ret : bool;
    if pos == 81 {
		return true
	}
	if sudoku_ar[pos] > 0 {
		ret = place_number( pos + 1, sudoku_ar );
		if ret == true {
			return true
		} else {
		    return false	
		}
	}
}


fn solve(sudoku_ar : SudokuArType){
	place_number( 0, sudoku_ar );
//    for elem  in  sudoku_ar.iter() {
//	    println!("elem {}",elem);
//	}	  	
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
