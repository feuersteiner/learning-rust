fn rotate_90(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            new_matrix[j][2 - i] = matrix[i][j]
        }
    }
    return new_matrix;
}

fn main() {
    println!("Hello, world!");
    assert_eq!(
        rotate_90([[1, 2, 3], [4, 5, 6], [7, 8, 9],]),
        [[7, 4, 1], [8, 5, 2], [9, 6, 3],]
    );
    assert_eq!(
        rotate_90([[1, 0, 0], [0, 0, 0], [0, 0, 2],]),
        [[0, 0, 1], [0, 0, 0], [2, 0, 0],]
    );
}
