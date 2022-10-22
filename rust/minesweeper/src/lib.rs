use itertools::Itertools;
use std::{collections::BTreeSet, fmt::Display, ops::Add};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Coord<T> {
    y: T,
    x: T,
}

impl TryFrom<&[&str]> for UncountedBoard {
    type Error = String;

    fn try_from(board: &[&str]) -> Result<Self, Self::Error> {
        let height = board.len();
        let width = if height == 0 { 0 } else { board[0].len() };
        let dimensions = Coord {
            x: width,
            y: height,
        };
        if board.iter().any(|row| row.len() != width) {
            return Err("Uneven row width".into());
        }
        let mines = board
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(cell_index, cell)| {
                        if cell == '*' {
                            Some(Coord {
                                x: cell_index,
                                y: row_index,
                            })
                        } else {
                            None
                        }
                    })
            })
            .collect();
        Ok(UncountedBoard { dimensions, mines })
    }
}

struct UncountedBoard {
    dimensions: Coord<usize>,
    mines: BTreeSet<Coord<usize>>,
}
struct CountedBoard(Vec<Vec<CountedBoardCell>>);
enum CountedBoardCell {
    Mine,
    Count(u8),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let uncounted_board: UncountedBoard = minefield.try_into().unwrap();
    let counted_board: CountedBoard = uncounted_board.into();
    counted_board.into()
}
impl From<UncountedBoard> for CountedBoard {
    fn from(uncounted_board: UncountedBoard) -> Self {
        Self(
            (0..uncounted_board.dimensions.y)
                .map(|row_index| -> Vec<CountedBoardCell> {
                    (0..uncounted_board.dimensions.x)
                        .map(|column_index| -> CountedBoardCell {
                            let coord = Coord {
                                x: column_index,
                                y: row_index,
                            };
                            if uncounted_board.mines.contains(&coord) {
                                CountedBoardCell::Mine
                            } else {
                                CountedBoardCell::Count(
                                    uncounted_board.count_neighboring_mines(&coord),
                                )
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }
}
impl UncountedBoard {
    fn count_neighboring_mines(&self, coord: &Coord<usize>) -> u8 {
        let range_x = coord.x.saturating_sub(1)..=coord.x.add(1).min(self.dimensions.x - 1);
        let range_y = coord.y.saturating_sub(1)..=coord.y.add(1).min(self.dimensions.y - 1);
        let adjacent_positions = range_x
            .cartesian_product(range_y)
            .map(|(x, y)| Coord { x, y });
        let adjacent_mines = adjacent_positions.filter(|coord| self.mines.contains(coord));
        adjacent_mines.count() as u8
    }
}

impl From<CountedBoard> for Vec<String> {
    fn from(counted_board: CountedBoard) -> Self {
        counted_board
            .0
            .into_iter()
            .map(|row| row.iter().map(ToString::to_string).collect())
            .collect()
    }
}

impl Display for CountedBoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell = match self {
            CountedBoardCell::Mine => String::from("*"),
            CountedBoardCell::Count(0) => String::from(" "),
            CountedBoardCell::Count(count) => count.to_string(),
        };
        write!(f, "{cell}")
    }
}
