const SCORE_GAP: isize = -4;
const SCORE_MISMATCH: isize = -3;
const SCORE_MATCH: isize = 1;

fn debug_matrix(seq_1: &String, seq_2: &String, matrix: Vec<Vec<isize>>)
{
    // Display first -
    print!(" \t-\t");

    // Display sequence 1 split with tabs
    for c in seq_1.chars()
    {
        print!("{}\t", c);
    }

    print!("\n-\t");

    // Display first row of matrix
    for col in &matrix[0]
    {
        print!("{}\t", col);
    }

    print!("\n");

    for (i, c) in seq_2.chars().enumerate()
    {
        print!("{}\t", c);

        for col in &matrix[i + 1]
        {
            print!("{}\t", col);
        }

        print!("\n");
    }
}

fn get_score(char_1: char, char_2: char) -> isize
{
    if char_1 == char_2
    {
        SCORE_MATCH
    } else {
        SCORE_MISMATCH
    }
}

fn new_matrix(x: usize, y: usize) -> Vec<Vec<isize>>
{
    let mut matrix: Vec<Vec<isize>> = vec![vec![0; x + 1]; y + 1];

    for n in 0..x + 1
    {
        matrix[0][n] = (n as isize) * SCORE_GAP;
    }

    for n in 0..y + 1
    {
        matrix[n][0] = (n as isize) * SCORE_GAP;
    }

    matrix
}

fn align(seq_1: &String, seq_2: &String) -> String
{
    let mut matrix: Vec<Vec<isize>> = new_matrix(seq_1.len(), seq_2.len());

    debug_matrix(&seq_1, &seq_2, matrix);

    String::new()
}

fn main()
{
    let seq_1: String = String::from("ACGGCTC");
    let seq_2: String = String::from("ATGGCCTC");

    let _map = align(&seq_1, &seq_2);
}
