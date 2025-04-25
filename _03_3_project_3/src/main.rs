// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    let opening = "On the [REPLACE] day of Christmas my true love sent to me, ";
    let verses = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    // adds "and " to the first verse
    let prefixed_first_verse = "and a partridge in a pear tree";

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];

    let days = days.iter().enumerate();

    for day in days {
        let (repitition, day) = day;

        // Opening line
        let opening = opening.replace("[REPLACE]", day);
        println!("{opening}");

        // Verses
        // let verses = verses.iter().enumerate().take(repitition + 1).rev();
        let verses = verses[..repitition + 1].iter().enumerate().rev();
        for verse in verses {
            let (verse_day, verse) = verse;
            // after opening, print each verse with (,) at end and finish with (.)
            // the first verse is always at the end because the verses iterator is reversed
            println!(
                "{}{}",
                if repitition > 0 && verse_day == 0 {
                    prefixed_first_verse
                } else {
                    verse
                },
                if verse_day == 0 { "." } else { "," }
            );
        }
        println!();
    }
}
