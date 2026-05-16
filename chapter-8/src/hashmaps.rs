use std::collections::HashMap;

pub fn part3(){
    let mut scores = HashMap::new();
 
    scores.insert("Blue", 10);
    scores.insert("Black", 12);
    scores.insert("Red", 9);

    println!("{scores:?}");


    let team_name = "Purple";
    scores.insert(team_name, 11);
    println!("{:?}", scores.get("Purple"));
    // println!("{:?}", scores.get("Red"));

    scores.insert(team_name, 10);
    println!("{:?}", scores.get("Purple"));

    /*
     *  value with Copy trait will be copied while others pass their ownership to HashMap 
     *  for example, str& will be copied while String will have its ownership passed
     *
     *  if a reference is given to hashmap, that reference must stay valid till hashmap is valid
     */

    // to update a value, you can just insert it again as it will overwrite

    // using or_insert
    scores.entry("Blue").or_insert(15); // will return mutable reference to the value

    // ######################################

    let text = "this is a word and word is word";
    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{word_map:?}");
}
