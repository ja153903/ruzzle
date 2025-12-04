pub const DIRECTIONS_WITH_DIAGONALS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn count_neighbors<T>(
    grid: &[Vec<T>],
    row: usize,
    col: usize,
    target: T,
    with_diagonals: bool,
) -> usize
where
    T: PartialEq,
{
    let directions: &[(i32, i32)] = if with_diagonals {
        &DIRECTIONS_WITH_DIAGONALS
    } else {
        &DIRECTIONS
    };

    directions
        .iter()
        .filter_map(|(dr, dc)| {
            let new_row = (row as i32 + dr) as usize;
            let new_col = (col as i32 + dc) as usize;

            grid.get(new_row)
                .and_then(|row| row.get(new_col))
                .filter(|&cell| *cell == target)
        })
        .count()
}
