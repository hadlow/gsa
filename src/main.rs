use std::cmp;

const SCORE_GAP: isize = -4;
const SCORE_MISMATCH: isize = -3;
const SCORE_MATCH: isize = 1;

fn debug_matrix(seq_1: &String, seq_2: &String, matrix: &Vec<Vec<isize>>)
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

fn new_traceback(x: usize, y: usize) -> Vec<Vec<char>>
{
    let mut matrix: Vec<Vec<char>> = vec![vec!['0'; x + 1]; y + 1];

    for n in 0..x + 1
    {
        matrix[0][n] = 'l';
    }

    for n in 0..y + 1
    {
        matrix[n][0] = 'u';
    }

    matrix[0][0] = 'e';

    matrix
}

fn align(seq_1: &String, seq_2: &String) -> (Vec<Vec<isize>>, Vec<Vec<char>>)
{
    let mut matrix: Vec<Vec<isize>> = new_matrix(seq_1.len(), seq_2.len());
    let mut traceback: Vec<Vec<char>> = new_traceback(seq_1.len(), seq_2.len());

    let mut l: isize = 0;
    let mut u: isize = 0;
    let mut d: isize = 0;

    for i in 1..seq_2.len() + 1
    {
        for j in 1..seq_1.len() + 1
        {
            l = matrix[i][j - 1] + SCORE_GAP;
            u = matrix[i - 1][j] + SCORE_GAP;
            d = matrix[i - 1][j - 1] + get_score(seq_2.chars().nth(i - 1).unwrap(), seq_1.chars().nth(j - 1).unwrap());

            matrix[i][j] = cmp::max(cmp::max(l, u), d);

            if matrix[i][j] == l
            {
				traceback[i][j] = 'l'
            } else if matrix[i][j] == u {
				traceback[i][j] = 'u'
            } else {
				traceback[i][j] = 'd'
            }
        }
    }

    (matrix, traceback)
}

fn render_alignment(seq_1: &String, seq_2: &String, traceback: &Vec<Vec<char>>)
{
    let mut x: Vec<char> = Vec::new();
    let mut y: Vec<char> = Vec::new();

    let mut i: usize = seq_2.len();
    let mut j: usize = seq_1.len();

    while i > 0 || j > 0
    {
        match traceback[i][j]
        {
            'l' => {
                x.push('-');
                y.push(seq_1.chars().nth(j - 1).unwrap());
                j = j - 1;
            },
            'u' => {
                x.push(seq_2.chars().nth(i - 1).unwrap());
                y.push('-');
                i = i - 1;
            },
            'd' => {
                x.push(seq_2.chars().nth(i - 1).unwrap());
                y.push(seq_1.chars().nth(j - 1).unwrap());
                i = i - 1;
                j = j - 1;
            },
            'e' => {
                break;
            },
            _ => {
                break;
            }
        }
    }

    while let Some(n) = x.pop() {
        print!("{}\t", n);
    }

    print!("\n");

    while let Some(n) = y.pop() {
        print!("{}\t", n);
    }

    print!("\n");
}

fn main()
{
    let seq_1: String = String::from("ACGGCTC");
    let seq_2: String = String::from("ATGGCCTC");

    let (matrix, traceback) = align(&seq_1, &seq_2);

    render_alignment(&seq_1, &seq_2, &traceback);
}
