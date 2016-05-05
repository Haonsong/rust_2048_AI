//! Skeleton crate for a 2048 AI.

extern crate lib_2048 ;

use lib_2048::Evolution ;
use lib_2048::rendering::{ Frame, rendering_loop } ;

use lib_2048::clap ;
use lib_2048::Grid ;
use lib_2048::Cell;


fn max_of_vec(score_board:Vec<usize>) -> (usize,usize){
  let mut index = 0usize;
  for i in 0..score_board.len(){
    if score_board[index] < score_board[i] {
      index = i;
    }
  }
  (index,score_board[index])
}

/// Sleeps for some time (in ms).
fn sleep(ms: u64) {
  use std::thread::sleep ;
  use std::time::Duration ;
  sleep(Duration::from_millis(ms)) ;
}

fn get_val_of_cell(cell:Option<Cell>)->usize{
  match cell{
    Some(cell)=> cell.val(),
    _ => 0,
  }
}

fn scoring(grid: & Grid)-> usize{
  //println!("in scoring") ;
  //let mut score =  0usize;
  let board = grid.grid().clone();
  //counters
  let mut empty_row_col =  0usize;
  // scores based on title
  let mut small_diff = 0usize;
  let mut same_cell = 0usize;
  let mut large_edge = 0usize;
  //---------------------------
  let mut max_cell = 0usize;
  let mut max_cell_row = 0usize;
  let mut max_cell_col = 0usize;
  //////////////////////////////////////////////////////////////////
  /////////////vertial zeros and same_cell count////////////////////
  //////////////////////////////////////////////////////////////////
  let mut row = 0;
  let mut col = 0;
  while  row < 4 {
    let mut zeros = 0usize;
    while  col < 3 {
      let c_cell = get_val_of_cell(board[row][col].clone());
      let r_cell = get_val_of_cell(board[row][col+1].clone());
      if max_cell < c_cell { max_cell = c_cell; }
      if max_cell < r_cell { max_cell = r_cell; }
      if c_cell != 0 {
        if c_cell == r_cell {
          col += 1;
          same_cell += c_cell;
        }
         if r_cell != 0 && ((c_cell/r_cell < 2) || (r_cell/c_cell < 2)){
           small_diff +=  if c_cell > r_cell {c_cell} else {r_cell}   ;
         }
      }
      if c_cell == 0 { zeros += 1};
      if r_cell == 0 { zeros += 1};
      col += 1;
    }
    if zeros == 4 {empty_row_col += 1 ;}
    col = 0;
    row +=1;
  }

  //////////////////////////////////////////////////////////////////
  /////////////vertial zeros and same_cell count////////////////////
  //////////////////////////////////////////////////////////////////
  row = 0;
  col = 0;
  while col < 4 {
    let mut zeros = 0usize;
    while row < 3 {
      let c_cell = get_val_of_cell(board[row][col].clone());
      let d_cell = get_val_of_cell(board[row+1][col].clone());
      if max_cell < c_cell { max_cell = c_cell; }
      if max_cell < d_cell { max_cell = d_cell; }
      if c_cell != 0 {
        if c_cell == d_cell {
          row += 1;
          same_cell += c_cell;
        }
        if d_cell != 0 && ((c_cell/d_cell < 2) || (d_cell/c_cell < 2)){
            small_diff +=  if c_cell > d_cell {c_cell} else {d_cell} ;
        }
      }
      if c_cell == 0 { zeros += 1};
      if d_cell == 0 { zeros += 1};
      row += 1;
    }
    if zeros == 4 {empty_row_col += 1 ;}
    row = 0;
    col+= 1;
  }


  //////////////////////////////////////////////////////////////////
  /////////////Large on edge////////////////////////////////////////
  //////////////////////////////////////////////////////////////////

  row = 0;
  col = 0;
  while col < 4 {
    while row < 4 {
      let c_cell = get_val_of_cell(board[row][col].clone());
      if c_cell == max_cell {
        max_cell_row = row;
        max_cell_col = col;
        if col == 0 || col == 3 {
          large_edge += 2*max_cell;
        }
        if row == 0 || row == 3 {
          large_edge += 2*max_cell;
        }
      }
      if c_cell < max_cell && c_cell > max_cell/4 {
        if col == 0 || col == 3 {
          large_edge += c_cell;
        }
        if row == 0 || row == 3 {
          large_edge += c_cell;
        }
      }
      row += 1;
    }
    col+= 1;
  }


  let mut max_near = 0usize;

  //////////////////////////////////////////////////////////////////
  /////////////Line up near large////////////////////////////////////////
  //////////////////////////////////////////////////////////////////

  let row_u = (max_cell_row)as i32 - 1;
  let row_d = (max_cell_row)as i32 + 1;
  let col_l = (max_cell_row)as i32 - 1;
  let col_r = (max_cell_row)as i32 + 1;
  if row_u >= 0 {
    for i in row_u as usize..0 {
        let c_cell = get_val_of_cell(board[i ][max_cell_col].clone());
        let p_cell = get_val_of_cell(board[i + 1][max_cell_col].clone());
        if c_cell < p_cell {
          max_near += c_cell *(4-( max_cell_row - i ));
        }
    }
  }
  if col_l >= 0 {
    for i in col_l as usize..0 {
        let c_cell = get_val_of_cell(board[max_cell_row][i].clone());
        let p_cell = get_val_of_cell(board[max_cell_row][i + 1].clone());
        if c_cell < p_cell {
          max_near += c_cell *(4 - ( max_cell_row - i ));
        }
    }
  }
  if row_d <= 3 {
    for i in row_d as usize..3 {
        let c_cell = get_val_of_cell(board[i ][max_cell_col].clone());
        let p_cell = get_val_of_cell(board[i - 1][max_cell_col].clone());
        if c_cell < p_cell {
          max_near += c_cell *(4-( i - max_cell_row  ));
        }
    }
  }
  if col_r <= 3 {
    for i in col_r as usize..3 {
        let c_cell = get_val_of_cell(board[max_cell_row][i].clone());
        let p_cell = get_val_of_cell(board[max_cell_row][i-1].clone());
        if c_cell < p_cell {
          max_near += c_cell *(4-( i - max_cell_col ));
        }
    }
  }






  //counters
  // let mut empty_row_col =  0usize;
  // scores based on title
  // let mut small_diff = 0usize;
  // let mut same_cell = 0usize;
  // let mut max_cell = 0usize;
  // let mut large_edge = 0usize;
  empty_row_col * max_cell/4 + small_diff/10 +  large_edge + same_cell  + grid.get_free().len()  * max_cell /4 + max_near
}

fn minmax (grid: & Grid,  depth: usize, rand_turn: bool)-> usize{
  let ref mut grid_copy = grid.clone();
  //insert_cell(&mut self, row: usize, col: usize, pow: u32) -> Option<Cell>
  //get_free(&self) -> Vec<(usize, usize)>
    if depth == 0 {
      //println!("scoring" );
      scoring(grid_copy)
    }else{
     if !rand_turn {
      let mut score_board = vec![2000,2000,2000,2000] ;
      let mut skip_board = vec![false,false,false,false] ;
      let mut next_grid = vec![
      grid_copy.clone(),
      grid_copy.clone(),
      grid_copy.clone(),
      grid_copy.clone() ] ;
      match next_grid[0].up(){
        Evolution::Nothing => skip_board[0] = true,
        _ =>{},
      };
      match next_grid[1].down(){
        Evolution::Nothing => skip_board[1] = true,
        _ =>{},
      };
      match next_grid[2].left(){
        Evolution::Nothing => skip_board[2] = true,
        _ =>{},
      };
      match next_grid[3].right(){
        Evolution::Nothing => skip_board[3] = true,
        _ =>{},
      };
      //recusively call minimax!
      for i in 0..score_board.len(){
        if skip_board[i] {
          score_board[i] = 0;
        }else{
          score_board[i] = minmax (& next_grid[i],depth - 1,true);
        }
      }
      //println!("I am in 4 -- direction turn depth {}", depth );
      match max_of_vec(score_board){
        (_,score) => score,
      }
    }else{
      let mut exp_score = 2000  * grid.get_free().len();
      for position in grid.get_free(){
        let (row,col) = position;
        let ref mut grid_new_2 = grid_copy.clone();
        let ref mut grid_new_4 = grid_copy.clone();
        grid_new_2.insert_cell(row,col,1);
        grid_new_4.insert_cell(row,col,2);
        exp_score += (0.8 * (minmax(grid_new_2,depth,false) as f64)) as usize;
        exp_score += (0.2 * (minmax(grid_new_4,depth,false) as f64)) as usize;
      }
      //println!("I am in rand_turn depth: {}", depth );
      exp_score / grid.get_free().len()
    }
  }
}

/// helper function for minimax
fn next_movement(grid:& Grid ) -> Vec<usize>{
  let mut score_board = vec![2000,2000,2000,2000] ;
  let mut skip_board = vec![false,false,false,false] ;
  let mut next_grid = vec![
  grid.clone(),
  grid.clone(),
  grid.clone(),
  grid.clone() ] ;

  match next_grid[0].up(){
    Evolution::Nothing => skip_board[0] = true,
    _ =>{},
  };
  match next_grid[1].down(){
    Evolution::Nothing => skip_board[1] = true,
    _ =>{},
  };
  match next_grid[2].left(){
    Evolution::Nothing => skip_board[2] = true,
    _ =>{},
  };
  match next_grid[3].right(){
    Evolution::Nothing => skip_board[3] = true,
    _ =>{},
  };

  for i in 0..score_board.len(){
    if skip_board[i] {
      score_board[i] = 0;
    }else{
      score_board[i] = minmax (& next_grid[i],3,true);
    }
  }
  score_board
}

/// Dumb AI that tries to go up, then left, then right, then down.
fn ai_move(frame: & mut Frame) -> Evolution {
  use std::process::exit;
  let ref grid_copy =frame.grid().clone();
  match max_of_vec(next_movement(grid_copy)) {
     (0,score) => {
       match frame.up(){
         Evolution::Nothing => {
           println!("I lost T_T") ;
           println!("score: {}",score) ;
           exit(0)
         },
         evol => evol,
       }
     },
     (1,score) => {
       match frame.down(){
         Evolution::Nothing => {
           println!("I lost T_T") ;
           println!("score: {}",score) ;
           exit(0)
         },
         evol => evol,
       }
     },
     (2,score) => {
       match frame.left(){
         Evolution::Nothing => {
           println!("I lost T_T") ;
           println!("score: {}",score) ;
           exit(0)
         },
         evol => evol,
       }
     },
     (3,score) => {
       match frame.right(){
         Evolution::Nothing => {
           println!("I lost T_T") ;
           println!("score: {}",score) ;
           exit(0)
         },
         evol => evol,
       }
     },
     _=>{
       println!("I lost T_T") ;
       println!("") ;
       exit(0)
     }
  }
}

fn main() {
  use std::process::exit ;
  // Getting seed and painter from command line arguments.
  let (seed, painter) = match clap::parse() {
    Ok( (seed, painter) ) => (seed, painter),
    Err( (e, painter) ) => {
      println!("{}\n> {}", painter.error("Error:"), e) ;
      exit(2)
    },
  } ;

  rendering_loop(seed, painter, ai_move)
}
