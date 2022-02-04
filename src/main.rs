use std::io;

fn main() {
    /*     let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 2, 6, 6, 7, 7, 7, 7];

       let vector: Vec<i32> = list.to_vec();

       //media

       let mut map = HashMap::new();
       //mode
       for i in list {
           let count = map.entry(i).or_insert(0);
           *count += 1;
       }
       println!("{:?}", map);
    */
    //convert strings to pig latin
    println!(
        "\n\nPIG LATIN CONVERTER\n--------------------- 
    Please input a word and I'll show you how to say it in Pig Latin:"
    );

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failure to read line.");

    let mut word = word.trim().to_string();

    //remove consonant and move it to the end
    //add "ay" at the end

    let mut letters = word.chars();

    let mut is_vowel: bool;

    let mut first_letter = letters.next();

    match first_letter {
        Some('a') => is_vowel = true,
        Some('e') => is_vowel = true,
        Some('i') => is_vowel = true,
        Some('o') => is_vowel = true,
        Some('u') => is_vowel = true,
        _ => is_vowel = false,
        None => {
            is_vowel = false;
            println!("You didn't input a word.");
            //once the loop is added, then add in the continue,
        }
    }

    if is_vowel {
        word.push_str("-hay");
    } else {
        let consonant = word.remove(0).to_string();
        word.push_str("-");
        word.push_str(consonant.as_str());
        word.push_str("ay");
    }

    println!("{:?}", word);
}

/*
fn main() {
    let teams = vec!
        String::from("Blue"),
        String::from("Yellow"),
        String::from("White"),
    ];

    let initial_scores = vec![10, 50, 0];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //accessing values in a hash map

    println!(
        "Accessing values in a hashmap via the get expression {:?}",
        scores.get("Blue")
    );

    //accessing values in hashmap with for loop

    for (a, b) in &scores {
        println!("{}: {}", a, b);
    }

    println!("For loop accessing items in hashmap {:?}", scores);

    //adding an item into a map

    scores.insert(String::from("Black"), 25);

    println!("Adding an item into the hashmap {:?}", scores);

    //only inserting a value if the key has no value

    scores.entry("Red".to_string()).or_insert(100);

    println!("Adding an item into the hashmap with entry {:?}", scores);

    //updating a value based on the old value
    let text = "hello world wonderful world world world hello world yes you are";

    let mut map = HashMap::new();

    //for each word in the sentence
    for word in text.split_whitespace() {
        //create a variable
        //enter the word from the sentence into the map if that word does not yet exist in the map
        //all values start at zero
        //if a word is added to the map, increase the value by 1
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
 */
