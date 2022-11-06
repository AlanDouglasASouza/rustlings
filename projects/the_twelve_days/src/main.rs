fn main() {
    println!("");
    println!("THE TWELVE DAYS OF CHRISTMAS");
    println!("");
    print_music();
}

fn print_music() {    

    let versus = [
        "A partridge in a pear tree.",
        "And a partridge in a pear tree.",
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
    ];
    let first_verse = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let initial_verse = "my true love sent to me";

    let mut i = 1;

    while i < 13 {

        println!("{}", build_first_verse(first_verse[i - 1]));
        println!("{}", initial_verse);

        if i == 1 {
            println!("{}", versus[0]);
        } else {
            let mut inc_versus = i;

            while inc_versus > 0 {
                println!("{}", versus[inc_versus]);
                inc_versus = inc_versus - 1;
            }
        }

        println!("");

        i = i + 1;
    }     
}

fn build_first_verse(word: &str) -> String {

    let initial: String = "On the ".to_owned();
    let finals = " day of Christmas,";
    let result = initial + word + finals;

    return result;
}
