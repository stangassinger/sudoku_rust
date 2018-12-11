

type SudokuArType = [u8; 81];


fn check_validity( val : u8, x : usize, y : usize, sudoku_ar : SudokuArType ) -> bool{
	for i in 0..8{
	    if sudoku_ar[y * 9 + i] == val || sudoku_ar[i * 9 + x] == val {
		    return false	
		}	
	}
	
	
	let startx : usize = ( x / 3 ) * 3;
	let starty : usize = ( y / 3 ) * 3;
	
	for i in starty.. starty + 2 {
	    for j in startx..startx + 2 {
		    if sudoku_ar[i * 9 + j] == val {
			    return false	
		    }	
	    }	
    }
    
    true
	
}



fn place_number( pos: usize, mut sudoku_ar: SudokuArType ) -> bool {
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
	for n in 1..9 {
	    if check_validity(n, pos % 9, pos / 9, sudoku_ar) == true {
		    sudoku_ar[pos] = n;
		    ret = place_number( pos + 1, sudoku_ar );
		    if ret == true{
			    return true;	
			}	
			sudoku_ar[pos] = 0;
		}    	
    }
    false
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
