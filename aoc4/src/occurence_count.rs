use std::sync::Arc;

fn is_xmas(first: u8, second: u8, third: u8, fourth: u8) -> bool {
    first == b'X' && second == b'M' && third == b'A' && fourth == b'S'
}

fn is_samx(first: u8, second: u8, third: u8, fourth: u8) -> bool {
    first == b'S' && second == b'A' && third == b'M' && fourth == b'X'
}

pub async fn count_occurrences_of_xmas(text: &str) -> u32 {
    let text: Arc<str> = Arc::from(text);

    let horizontal = tokio::spawn({
        let text = text.clone();

        async move { count_occurrences_of_xmas_horizontal(&text) }
    });

    let vertical = tokio::spawn({
        let text = text.clone();

        async move { count_occurrences_of_xmas_vertical(&text) }
    });

    let diagonal = tokio::spawn({
        let text = text.clone();

        async move { count_occurrences_of_xmas_diagonal(&text) }
    });

    horizontal.await.unwrap() + vertical.await.unwrap() + diagonal.await.unwrap()
}

pub fn count_occurrences_of_xmas_horizontal(text: &str) -> u32 {
    let mut occurrences = 0;

    for line in text.lines() {
        for slice in line.as_bytes().windows(4) {
            if is_xmas(slice[0], slice[1], slice[2], slice[3])
                || is_samx(slice[0], slice[1], slice[2], slice[3])
            {
                occurrences += 1;
            }
        }
    }

    occurrences
}

pub fn count_occurrences_of_xmas_vertical(text: &str) -> u32 {
    let mut occurrences = 0;

    let lines = text
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    for line in lines.windows(4) {
        for i in 0..line[0].len() {
            if is_xmas(line[0][i], line[1][i], line[2][i], line[3][i])
                || is_samx(line[0][i], line[1][i], line[2][i], line[3][i])
            {
                occurrences += 1;
            }
        }
    }

    occurrences
}

pub fn count_occurrences_of_xmas_diagonal(text: &str) -> u32 {
    let mut occurrences = 0;

    let lines = text
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    for line in lines.windows(4) {
        for i in 0..line[0].len() - 3 {
            // left to right
            if is_xmas(line[0][i], line[1][i + 1], line[2][i + 2], line[3][i + 3])
                || is_samx(line[0][i], line[1][i + 1], line[2][i + 2], line[3][i + 3])
            {
                occurrences += 1
            }

            if is_xmas(line[0][i + 3], line[1][i + 2], line[2][i + 1], line[3][i])
                || is_samx(line[0][i + 3], line[1][i + 2], line[2][i + 1], line[3][i])
            {
                occurrences += 1
            }
        }
    }

    occurrences
}
