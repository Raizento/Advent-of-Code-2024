pub fn distance(start: i32, end: i32) -> i32 {
    (start - end).abs()
}

pub fn tuple_distance(tuple: (&i32, &i32)) -> i32 {
    (tuple.0 - tuple.1).abs()
}

pub fn total_distance(left: &mut [i32], right: &mut [i32]) -> i32 {
    left.sort();
    right.sort();

    let zip = left.iter().zip(right.iter());
    let total_distance: i32 = zip.map(tuple_distance).sum();
    total_distance
}
