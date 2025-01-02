use std::sync::Arc;

pub async fn count_occurrences_of_xmas(text: &str) -> u32 {
    let text: Arc<str> = Arc::from(text);

    let horizontal = tokio::spawn({
        let text = text.clone();

        async move {
            count_occurrences_of_xmas_horizontal(&text)
        }
    });

    let vertical = tokio::spawn({
        let text = text.clone();

        async move {
            count_occurrences_of_xmas_vertical(&text)
        }
    });

    let diagonal = tokio::spawn({
        let text = text.clone();

        async move {
            count_occurrences_of_xmas_diagonal(&text)
        }
    });

    horizontal.await.unwrap() + vertical.await.unwrap() + diagonal.await.unwrap()
}

pub fn count_occurrences_of_xmas_horizontal(text: &str) -> u32 {
    let mut occurrences = 0;

    for line in text.lines() {
        for slice in line.as_bytes().windows(4) {
            if slice[0] == b'X' && slice[1] == b'M' && slice[2] == b'A' && slice[3] == b'S' {
                occurrences += 1;
            }

            if slice[0] == b'S' && slice[1] == b'A' && slice[2] == b'M' && slice[3] == b'X' {
                occurrences += 1;
            }
        }
    }

    occurrences
}
 
pub fn count_occurrences_of_xmas_vertical(text: &str) -> u32 {
    let mut occurrences = 0;

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
                occurrences += 1;
            }

            if line[0][i] == b'S' && line[1][i] == b'A' && line[2][i] == b'M' && line[3][i] == b'X'
            {
                occurrences += 1;
            }
        }
    }

    occurrences
}

pub fn count_occurrences_of_xmas_diagonal(text: &str) -> u32 {
    let mut occurrences = 0;

    let lines = text.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    for window in lines.windows(4) {
        for i in 0..window[0].len()-3 {
            // left to right
            if window[0][i] == b'X'
            && window[1][i+1] == b'M'
            && window[2][i+2] == b'A'
            && window[3][i+3] == b'S' {
                occurrences += 1;
            }

            if window[0][i] == b'S'
            && window[1][i+1] == b'A'
            && window[2][i+2] == b'M'
            && window[3][i+3] == b'X' {
                occurrences += 1;
            }

            // right to left

            if window[0][i+3] == b'X'
            && window[1][i+2] == b'M'
            && window[2][i+1] == b'A'
            && window[3][i] == b'S' {
                occurrences += 1;
            }

            if window[0][i+3] == b'S'
            && window[1][i+2] == b'A'
            && window[2][i+1] == b'M'
            && window[3][i] == b'X' {
                occurrences += 1;
            }
        }
    }

    occurrences
}
