pub fn count_occurences_of_xmas(text: &str) -> u32 {
    0
}

pub fn count_occurences_of_xmas_vertical(text: &str) -> u32 {
    let mut occurences = 0;

    if text.lines().count() < 4 {
        return 0;
    }

    let lines = text
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    for line in lines.windows(4) {
        for i in 0..line[0].len() {
            if line[0][i] == b'X' && line[1][i] == b'M' && line[2][i] == b'A' && line[3][i] == b'S'
            {
                occurences += 1;
            }

            if line[0][i] == b'S' && line[1][i] == b'A' && line[2][i] == b'M' && line[3][i] == b'X'
            {
                occurences += 1;
            }
        }
    }

    occurences
}

pub fn count_occurences_of_xmas_horizontal(text: &str) -> u32 {
    let mut occurences = 0;

    for line in text.lines() {
        for slice in line.as_bytes().windows(4) {
            if slice[0] == b'X' && slice[1] == b'M' && slice[2] == b'A' && slice[3] == b'S' {
                occurences += 1;
            }

            if slice[0] == b'S' && slice[1] == b'A' && slice[2] == b'M' && slice[3] == b'X' {
                occurences += 1;
            }
        }
    }

    occurences
}

pub fn count_occurences_of_xmas_diagonal(text: &str) -> u32 {
    let mut occurences = 0;

    let lines = text.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    for window in lines.windows(4) {
        for i in 0..window[0].len()-3 {
            // left to right
            if window[0][i] == b'X'
            && window[1][i+1] == b'M'
            && window[2][i+2] == b'A'
            && window[3][i+3] == b'S' {
                occurences += 1;
            }

            if window[0][i] == b'S'
            && window[1][i+1] == b'A'
            && window[2][i+2] == b'M'
            && window[3][i+3] == b'X' {
                occurences += 1;
            }

            // right to left

            if window[0][i+3] == b'X'
            && window[1][i+2] == b'M'
            && window[2][i+1] == b'A'
            && window[3][i] == b'S' {
                occurences += 1;
            }

            if window[0][i+3] == b'S'
            && window[1][i+2] == b'A'
            && window[2][i+1] == b'M'
            && window[3][i] == b'X' {
                occurences += 1;
            }
        }
    }

    occurences
}

#[test]
fn test_count_occurences_of_xmas_horizontal_single_xmas_forward() {
    let text = "XMAS";

    let occurences = count_occurences_of_xmas_horizontal(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_horizontal_single_xmas_backward() {
    let text = "SAMX";

    let occurences = count_occurences_of_xmas_horizontal(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_horizontal_multiple_xmas() {
    let text = "MMMSXXMASM\n\
                       MSAMXMSMSA\n\
                       AMXSXMAAMM\n\
                       MSAMASMSMX\n\
                       XMASAMXAMM\n\
                       XXAMMXXAMA\n\
                       SMSMSASXSS\n\
                       SAXAMASAAA\n\
                       MAMMMXMMMM\n\
                       MXMXAXMASX";

    let occurences = count_occurences_of_xmas_horizontal(text);

    assert_eq!(5, occurences);
}

#[test]
fn test_count_occurences_of_xmas_vertical_single_xmas_forward() {
    let text = "X\n\
                      M\n\
                      A\n\
                      S";

    let occurences = count_occurences_of_xmas_vertical(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_vertical_single_xmas_backward() {
    let text = "S\n\
                      A\n\
                      M\n\
                      X";

    let occurences = count_occurences_of_xmas_vertical(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_vertical_multiple_xmas() {
    let text = "MMMSXXMASM\n\
                       MSAMXMSMSA\n\
                       AMXSXMAAMM\n\
                       MSAMASMSMX\n\
                       XMASAMXAMM\n\
                       XXAMMXXAMA\n\
                       SMSMSASXSS\n\
                       SAXAMASAAA\n\
                       MAMMMXMMMM\n\
                       MXMXAXMASX";

    let occurences = count_occurences_of_xmas_vertical(text);

    assert_eq!(3, occurences);
}

#[test]
fn test_count_occurences_of_xmas_diagonal_single_xmas_forward() {
    let text = "X...\n\
                      .M..\n\
                      ..A.\n\
                      ...S";

    let occurences = count_occurences_of_xmas_diagonal(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_diagonal_single_xmas_backward() {
    let text = "S...\n\
                      .A..\n\
                      ..M.\n\
                      ...X";

    let occurences = count_occurences_of_xmas_diagonal(text);

    assert_eq!(1, occurences);
}

#[test]
fn test_count_occurences_of_xmas_diagonal_multiple_xmas() {
    let text = "MMMSXXMASM\n\
                       MSAMXMSMSA\n\
                       AMXSXMAAMM\n\
                       MSAMASMSMX\n\
                       XMASAMXAMM\n\
                       XXAMMXXAMA\n\
                       SMSMSASXSS\n\
                       SAXAMASAAA\n\
                       MAMMMXMMMM\n\
                       MXMXAXMASX";

    let occurences = count_occurences_of_xmas_diagonal(text);

    assert_eq!(10, occurences);
}
