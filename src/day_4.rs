struct Board {
  board: Vec<Vec<i64>>,
  won: bool
}

impl Board {
  fn place(&mut self, nr: i64) -> bool {
    if self.won {
      return false;
    }
    self.board.iter_mut()
      .flat_map(|v| v.iter_mut())
      .filter(|n| **n == nr)
      .for_each(|n| *n += 1000);

    let row_win = self.board.iter().any(|row| row.iter().sum::<i64>() > 5000);
    let column_win = (0..5).into_iter().any(|i| {
      self.board[0][i] + self.board[1][i] + self.board[2][i] + self.board[3][i] + self.board[4][i] > 5000
    });

    self.won = row_win || column_win;
    self.won
  }

  fn get_score(&self) -> i64 {
    self.board.iter()
      .flat_map(|v| v.iter())
      .filter(|n| **n < 1000)
      .sum()
  }
}

pub fn a(input: &str) -> i64 {
  let lines: Vec<String> = input
    .lines()
    .filter(|s| !s.is_empty())
    .map(|s|s.to_string())
    .collect();

  let draw_numbers: Vec<i64> = lines[0]
    .split_terminator(',')
    .map(|s| s.parse().unwrap())
    .collect();

  let mut boards: Vec<Board> = lines[1..]
    .chunks(5)
    .map(|board_rows| {
      let board: Vec<Vec<i64>> = board_rows.iter()
        .map(|row|
          row
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
        ).collect();
      
      Board{board, won: false}
    })
    .collect();

  for number in draw_numbers {
    for board in &mut boards {
      if board.place(number) {
        return board.get_score() * number;
      };
    }
  }
  0
}

pub fn b(input: &str) -> i64 {
  let lines: Vec<&str> = input
    .lines()
    .filter(|s| !s.is_empty())
    .collect();

  let draw_numbers: Vec<i64> = lines[0]
    .split_terminator(',')
    .map(|s| s.parse().unwrap())
    .collect();

  let mut boards: Vec<Board> = lines[1..]
    .chunks(5)
    .map(|board_rows| {
      let board: Vec<Vec<i64>> = board_rows.iter()
        .map(|row|
          row
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
        ).collect();
      
      Board{board, won:false}
    })
    .collect();

  let mut last_winner_score = 0;
  for number in draw_numbers {
    for board in &mut boards {
      if board.place(number) {
        last_winner_score = board.get_score() * number;
      };
    }
  }
  last_winner_score
}