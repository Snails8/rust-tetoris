mod block;

use std::{thread, time};
use block::{BlockKind, BLOCKS};


struct Position {
  x: usize,
  y: usize,
}

// フィールドサイズ
const FIELD_WIDTH:  usize = 11 + 2;  // フィールド＋壁
const FIELD_HEIGHT: usize = 20 + 1;  // フィールド＋底
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

fn is_collection(field: &Field, pos: &Position, block: BlockKind) -> bool {
  for y in 0..4 {
    for x in 0..4 {
      if field[y+pos.y+1][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
        return true
      }
    }
  }
  return false;
}

fn main() {

  let field = [
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1],
  ];

  let mut pos = Position { x: 4, y: 0 };

  // 画面クリア
  println!("\x1b[2J\x1b[H\x1b[?25l");

  // block info
  for _ in 0..30 {
    let mut field_buf = field; // mutable
    if !is_collection(&field, &pos, BlockKind::I) {
      pos.y += 1;
    }

    for y in 0..4 {
      for x in 0..4 {
        // field_buf[y+ 2][x+2] |= BLOCKS[BlockKind::I as usize][y][x];
        // field_buf[y+ 2][x+7] |= BLOCKS[BlockKind::O as usize][y][x];
        // field_buf[y+ 6][x+2] |= BLOCKS[BlockKind::S as usize][y][x];
        // field_buf[y+ 6][x+7] |= BLOCKS[BlockKind::Z as usize][y][x];
        // field_buf[y+10][x+2] |= BLOCKS[BlockKind::J as usize][y][x];
        // field_buf[y+10][x+7] |= BLOCKS[BlockKind::L as usize][y][x];
        // field_buf[y+14][x+2] |= BLOCKS[BlockKind::T as usize][y][x];

        if BLOCKS[BlockKind::I as usize][y][x] == 1 {
          field_buf[y+pos.y][x+pos.x] = 1;
        }
      }
    }

    // update pos
    pos.y += 1;

    // show field
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
      for x in 0..FIELD_WIDTH {
        if field_buf[y][x] == 1 {
          print!("[]");
        } else {
          print!(" .")
        }
      }
      println!(); // 改行の役目
    }
    
    thread::sleep(time::Duration::from_millis(1000));
  }

  // カーソルを再表示
  println!("\x1b[?25h");
}