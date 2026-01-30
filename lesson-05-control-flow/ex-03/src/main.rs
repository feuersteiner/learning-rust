fn decide_cell(grid_item: Option<i32>) -> (bool, i32) {
    let result = match grid_item {
        Some(-1) => 0,
        None => 0,
        Some(i) => i,
    };
    if grid_item == Some(-1) {
        (false, result)
    } else {
        (true, result)
    }
}

fn hunt_treasure(grid: [[Option<i32>; 4]; 4]) -> (i32, usize, usize) {
    let mut treasure = 0;

    let mut last_row = 0;
    let mut last_col = 0;
    'outer: for i in 0..4 {
        for j in (0..4).rev() {
            let (decision, amount) = decide_cell(grid[i][j]);
            last_row = i;
            last_col = j;
            if !decision {
                println!("breaking at r[{}] c[{}]", i + 1, j + 1);

                break 'outer;
            } else {
                treasure += amount;
            }
        }
    }
    return (treasure, last_row, last_col);
}

fn main() {
    assert_eq!(decide_cell(Some(-1)), (false, 0));

    assert_eq!(decide_cell(Some(5)), (true, 5));
    assert_eq!(decide_cell(None), (true, 0));

    // Test hunt_treasure with grids
    let grid_no_traps: [[Option<i32>; 4]; 4] = [
        [Some(1), Some(2), Some(3), Some(4)],
        [Some(1), Some(1), Some(1), Some(1)],
        [None, None, None, None],
        [Some(5), Some(5), Some(5), Some(5)],
    ];
    assert_eq!(hunt_treasure(grid_no_traps), (34, 3, 0)); // all cells, last position

    let grid_trap_middle: [[Option<i32>; 4]; 4] = [
        [Some(1), Some(2), Some(3), Some(4)], // row scans right-to-left: 4,3,2,1 = 10
        [Some(1), Some(-1), Some(1), Some(1)], // hits trap at col 1
        [None, None, None, None],
        [Some(5), Some(5), Some(5), Some(5)],
    ];
    assert_eq!(hunt_treasure(grid_trap_middle), (12, 1, 1)); // stops at row 1, col 1

    let grid_trap_first: [[Option<i32>; 4]; 4] = [
        [Some(1), Some(2), Some(3), Some(-1)], // trap immediately (col 3)
        [Some(1), Some(1), Some(1), Some(1)],
        [None, None, None, None],
        [Some(5), Some(5), Some(5), Some(5)],
    ];
    assert_eq!(hunt_treasure(grid_trap_first), (0, 0, 3)); // stops immediately

    println!("All tests passed!");
}
