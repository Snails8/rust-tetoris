use std::{thread, time};

struct Position {
  x: usize,
  y: usize,
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
  for _ in 0..5 {
    let mut field_buf = field; // mutable
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
    for y in 0..21 {
      for x in 0..13 {
        if field_buf[y][x] == 1 {
          print!("[]");  // 
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

enum BlockKind {
  I,
  O,
  S,
  Z,
  J,
  L,
  T,
}

type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
  // Iブロック
  [
    [0,0,0,0],
    [0,0,0,0],
    [1,1,1,1],
    [0,0,0,0],
  ],
  // Oブロック
  [
    [0,0,0,0],
    [0,1,1,0],
    [0,1,1,0],
    [0,0,0,0],
  ],
  // Sブロック
  [
    [0,0,0,0],
    [0,1,1,0],
    [1,1,0,0],
    [0,0,0,0],
  ],
  // Zブロック
  [
    [0,0,0,0],
    [1,1,0,0],
    [0,1,1,0],
    [0,0,0,0],
  ],
  // Jブロック
  [
    [0,0,0,0],
    [1,0,0,0],
    [1,1,1,0],
    [0,0,0,0],
  ],
  // Lブロック
  [
    [0,0,0,0],
    [0,0,1,0],
    [1,1,1,0],
    [0,0,0,0],
  ],
  // Tブロック
  [
    [0,0,0,0],
    [0,1,0,0],
    [1,1,1,0],
    [0,0,0,0],
  ],
];