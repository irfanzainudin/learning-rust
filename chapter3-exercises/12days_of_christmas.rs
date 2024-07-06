fn repeating_lines(new_lines: Vec<&str>) {
    println!("my true love gave to me");
    for nl in &new_lines {
        println!("{nl}");
    }
    println!("And a partridge in a pear tree!\n");
}

fn main() {
    // 1st
    println!("On the first day of Christmas,");
    println!("my true love gave to me");
    println!("And a partridge in a pear tree!\n");
    // 2nd
    println!("On the second day of Christmas,");
    repeating_lines(vec!["Two turtle doves,"]);
    // 3rd
    println!("On the second day of Christmas,");
    repeating_lines(vec!["Two turtle doves,", "Three French hens,"]);
    // 4th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
    ]);
    // 5th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,"
    ]);
    // 6th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,"
    ]);
    // 7th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,"
    ]);
    // 8th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,"
    ]);
    // 9th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,"
    ]);
    // 10th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,"
    ]);
    // 11th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,"
    ]);
    // 12th
    println!("On the second day of Christmas,");
    repeating_lines(vec![
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ]);
}

// Full lyrics for reference:
//
// On the first day of Christmas,
// my true love gave to me
// A partridge in a pear tree.
// 
// On the second day of Christmas,
// my true love gave to me
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the third day of Christmas,
// my true love gave to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the fourth day of Christmas,
// my true love gave to me
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the fifth day of Christmas,
// my true love gave to me
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the sixth day of Christmas,
// my true love gave to me
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the seventh day of Christmas,
// my true love gave to me
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the eighth day of Christmas,
// my true love gave to me
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the ninth day of Christmas,
// my true love gave to me
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the tenth day of Christmas,
// my true love gave to me
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the eleventh day of Christmas,
// my true love gave to me
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// 
// On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!