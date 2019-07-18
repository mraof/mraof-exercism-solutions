pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        n => format!("{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", bottles(n), bottles(n), bottles(n - 1))
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}

fn bottles(n: i32) -> String {
    match n {
        1 => "1 bottle".to_string(),
        n => format!("{} bottles", n)
    }
}