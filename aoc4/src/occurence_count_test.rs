use crate::occurence_count::count_occurrences_of_xmas;

#[tokio::test]
async fn test_count_occurrences_of_xmas() {
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

    let occurrences = count_occurrences_of_xmas(text).await;

    assert_eq!(18, occurrences);
}

mod horizontal {
    use crate::occurence_count::count_occurrences_of_xmas_horizontal;

    #[test]
    fn test_count_occurrences_of_xmas_horizontal_single_xmas_forward() {
        let text = "XMAS";

        let occurrences = count_occurrences_of_xmas_horizontal(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_horizontal_single_xmas_backward() {
        let text = "SAMX";

        let occurrences = count_occurrences_of_xmas_horizontal(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_horizontal_multiple_xmas() {
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

        let occurrences = count_occurrences_of_xmas_horizontal(text);

        assert_eq!(5, occurrences);
    }
}

mod vertical {
    use crate::occurence_count::count_occurrences_of_xmas_vertical;

    #[test]
    fn test_count_occurrences_of_xmas_vertical_single_xmas_forward() {
        let text = "X\n\
                          M\n\
                          A\n\
                          S";

        let occurrences = count_occurrences_of_xmas_vertical(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_vertical_single_xmas_backward() {
        let text = "S\n\
                          A\n\
                          M\n\
                          X";

        let occurrences = count_occurrences_of_xmas_vertical(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_vertical_multiple_xmas() {
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

        let occurrences = count_occurrences_of_xmas_vertical(text);

        assert_eq!(3, occurrences);
    }
}

mod diagonal {
    use crate::occurence_count::count_occurrences_of_xmas_diagonal;

    #[test]
    fn test_count_occurrences_of_xmas_diagonal_single_xmas_forward() {
        let text = "X...\n\
                          .M..\n\
                          ..A.\n\
                          ...S";

        let occurrences = count_occurrences_of_xmas_diagonal(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_diagonal_single_xmas_backward() {
        let text = "S...\n\
                          .A..\n\
                          ..M.\n\
                          ...X";

        let occurrences = count_occurrences_of_xmas_diagonal(text);

        assert_eq!(1, occurrences);
    }

    #[test]
    fn test_count_occurrences_of_xmas_diagonal_multiple_xmas() {
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

        let occurrences = count_occurrences_of_xmas_diagonal(text);

        assert_eq!(10, occurrences);
    }
}

mod mas {
    use crate::occurence_count::count_occurences_of_cross_mas;

    #[test]
    fn test_count_occurrences_of_cross_mas() {
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

        let occurrences = count_occurences_of_cross_mas(text);

        assert_eq!(9, occurrences)
    }
}
