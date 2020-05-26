fn main() {
    let mut index: i32 = 1;
    let mut lyrics = String::new();

    while index < 13 {
        match index {
            1 => lyrics.push_str("A Partridge in a Pear Tree"),
            2 => lyrics.insert_str(0, "Two Turtle Doves,\nand "),
            3 => lyrics.insert_str(0, "Three French Hens,\n"),
            4 => lyrics.insert_str(0, "Four Calling Birds,\n"),
            5 => lyrics.insert_str(0, "Five Golden Rings,\n"),
            6 => lyrics.insert_str(0, "Six Geese a Laying,\n"),
            7 => lyrics.insert_str(0, "Seven Swans a Swimming,\n"),
            8 => lyrics.insert_str(0, "Eight Maids a Milking,\n"),
            9 => lyrics.insert_str(0, "Nine Ladies Dancing,\n"),
            10 => lyrics.insert_str(0, "Ten Lords a Leaping,\n"),
            11 => lyrics.insert_str(0, "Eleven Pipers Piping,\n"),
            12 => lyrics.insert_str(0, "12 Drummers Drumming,\n"),
            _ => return,
        }
        println!(
            "On the {} day of Christmas, my true love sent to me:\n{}",
            index, lyrics
        );
        index += 1;
    }
}
