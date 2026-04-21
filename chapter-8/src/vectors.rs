use std::{io, vec};

pub fn part1() {

    let _vector: Vec<i32> = Vec::new();
    let mut vector = vec![1,2,3]; // default vector to initialize vector with values. default type: i32
    
    println!("{vector:?}");
    
    vector.push(1);
    println!("{vector:?}");
    
    println!("{}", &vector[2]); // accessing the index using reference
    
    
    println!("enter index: ");
    let mut index_input = String::new();
    
    
    io::stdin()
    .read_line(&mut index_input)
    .expect("error while reading input");

    let index_input: usize = match index_input.trim().parse() {
        Ok(num) => { num },
        Err(_) => { println!("index cannot be negative. using 0 index instead"); 0 },
    };

    let index = vector.get(index_input); // provides optional
    println!("{index:?}");

    match index {
        Some(index) => println!("{index}"),
        None => println!("invalid index. out of bound")
    }

    /*
    when we store a reference and try to modify the vector in same scope,
    we can't do that. it is because of how vectos work.
    for example, if we push code and the resize occurs, the stored
    reference is invalid. its a possibility when we are trying to access the reference,
    it might give ambiguous result.
    */
    // let first = &vector[0];
    // vector.push(6);
    // println!("The first element is: {first}");


    let mut v = vec![67, 69, 420, 911];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // need to dereference to get to the value, then modify
    }

    #[derive(Debug)]
    enum CropType {
        RICE,
        WHEAT,
        SUGARCANE
    }

    struct Crop {
        name: String,
        crop_type: CropType
    }

    impl Crop {
        fn new(name: &str, crop_type: CropType) -> Crop {
            Crop {
                name: name.to_string(),
                crop_type
            }
        }
        
        fn display(&self) {
            println!("name: {}", self.name);
            println!("name: {0:?}", self.crop_type);
        }
    }

    let garden = vec![
        Crop::new("Wheat", CropType::WHEAT),
        Crop::new("Rice", CropType::RICE),
        Crop::new("Sugar Cane", CropType::SUGARCANE)
        ];
        
        let crop1 = &garden[0];
        crop1.display();
}