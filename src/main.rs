

type SudokuArType = [u8; 81];


fn check_validity( val : u8, x : usize, y : usize, sudoku_ar : SudokuArType ) -> bool{
	for i in 0..8 {
	    if ( sudoku_ar[y * 9 + i] == val ) || ( sudoku_ar[i * 9 + x] == val ) {
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



fn place_number( pos: usize, sudoku_ar: &mut SudokuArType ) -> bool {
	let mut ret : bool;
    if pos == 81 {
		return true
	}
	
	
	if sudoku_ar[pos] > 0 {
		ret = place_number( pos + 1,  sudoku_ar );
		if ret == true {
			return true
		} else {
		    return false	
		}
	}
	for n in 1..9 {
	    if check_validity(n, pos % 9, pos / 9, *sudoku_ar) == true {
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



fn pretty_print( sudoku_ar : SudokuArType ) {
	let line_sep = "------+------+------";
	println!("{}",line_sep);
	for i in 0..sudoku_ar.len() {		
	    print!("{} ", sudoku_ar[i] );
	    if (i + 1) % 3 == 0 && !( (i + 1) % 9 == 0 ) {
		    print!("| ");	
		}
		if (i+1) % 9 == 0 {
            println!(" ");
	    } 
	    if (i+1) % 27 == 0 {
            println!("{}", line_sep);
        }
	}
}



fn solve( sudoku_ar : &mut SudokuArType) -> bool {
	place_number( 0,  sudoku_ar )	
}


fn main() {
     let mut sudoku_ar: SudokuArType = [
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

    
     
     
     if solve( &mut sudoku_ar ) == false {
         println!("Unsolvable");        
     } else {
         pretty_print( sudoku_ar );
     }      
             
}
