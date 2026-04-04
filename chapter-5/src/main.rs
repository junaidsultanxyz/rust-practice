
struct User {
    username: String,
    email: String,
    password: String,
    active: bool,
}

/*
    tuple struct. not having field names but having overall name to differentiate from other tuples.

    even if other named tuples may have same argument and type, we can't assign one from other's value.
    for example, Color (i32, i32, i32) cannot take Point(i32, i32, i32).
*/
struct Point(i32, i32, i32);

/*
    can also define empty structs.
    these are called Unit-Like Structs.

    behave like ().
*/
struct Empty;

fn main() {

    // order of fields to initialize is not strict
    let user1 = User {
        username: String::from("junaidxyz"),
        email: String::from("junaidsultan@gmail.com"),
        password: String::from("123456"),
        active: true
    };
    print_user(&user1);

    let mut user2 = User {
        username: String::from("abdullahxyz"),
        email: String::from("abdullahxyz@gmail.com"),
        password: String::from("12345678"),
        active: false 
    };

    user2.active = true; // to change the values, the whole struct object must be mutable
    print_user(&user2);
    
    // user1.active = false; // this will cause error as user1 is not mutable


    let user3: User = build_user
        ("ahmad", "ahmad@gmail.com", "786786");

    print_user(&user3);

    /*
        we can also use another struct objects value to initialize new struct object
    */
    let user4 = User {
        password: String::from("123123"),
        username: user2.username,
        ..user3 // map the remaining fields (email, active) to user3 fields (email, active)
    //  ^^^^^^^ the remaining must also come at the last
    };

    /*
        trying to print user2 and user3 wont compile as user.username and remaining values of user3
        were moved to user4.

        if we did user2.username.clone(), it wont give error as that gives an actual copy to user4 instead of moving the value.
        but if we print the other values that have the Copy trait, they can be printed.
    */
    // print_user(&user2);
    // print_user(&user3);
    
    print_user(&user4);


    let position = Point(12, -10, 11);
    print_point(&position);

    /*
        this destructures the position value into the x,y,z variables.
        but to do this, we must mention the tuple type.
    */
    let Point(x,y,z) = position;
    print_point(&position);

    /*
        and since these types have the copy trait, they will be copied and not moved.
    */
    println!("destructured point: ({x}, {y}, {z})");


    let store = Empty;
}


fn print_user (user: &User) {
    println!("User ({0}, {1}, {2}, {3})", user.username, user.email, user.password, user.active);
}

fn print_point (val: &Point) {
    println!("Point ({0}, {1}, {2})", val.0, val.1, val.2);
}


fn build_user (username: &str, email: &str, password: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        password: String::from(password),
    }
}


/*
    we can use the shorthand to initialize where if the parameter name is matching
    the struct field name, we can directly pass it for initialization.

    we dont have to write email: email or username: username.
    just email and username are good.
*/
fn build_user_example (username: String, email: String, password: String) -> User {
    User {
        active: true,
        username,
        password,
        email
    }
}