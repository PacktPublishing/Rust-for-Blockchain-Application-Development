// represent a block from a blockchain, using Rust structs

pub struct Block { 
    pub id: u64, 
    pub hash: String, 
    pub previous_hash: String, 
    pub timestamp: i64, 
    pub txn_data: String, 
    pub nonce: u64, 
}


// blockchain can be represented
pub struct Blockchain { 
    pub blocks: Vec, 
} 

// variables in rust
fn main() { 
    let x = 5; 
    println!(“The value of is:{x}”); 
} 

//  all variables in Rust are immutable
fn main() { 
    let x = 5; 
    println!(“The value of is:{x}”); 
    let x = 6; 
    println!(“The value of is:{x}”); 
} 

// “mut” keyword
fn main() { 
    let mut x = 5; 
    println!(“The value of is:{x}”); 
    x = 6; 
    println!(“The value of is:{x}”); 
} 

// a constant expression
    Const HOURS_IN_A_DAY = 24; 


// “let” keyword to assign a new value to the variable 
fn main() { 
    let mut x = 5; 
    println!(“The value of is:{x}”); 
    let x = 6; 
    println!(“The value of is:{x}”); 
} 


//  floating-point numbers
fn main() { 
    let x = 2.0; 
    let y: f32 = 3.0 
} 

// Boolean
fn main() { 
    let t = true; 
    let f: bool = false; 
} 

// char
fn main() { 
    let t: char = 'z'; 
    let f: char = Smiley face emoji ; 
} 

// tuples 
 fn main() { 
    let tup = (500, 6.4, 1); 
    let (x, y, z) = tup; 
    println!(“the value of y is: {y}”); 
    let x:(i32, f64, u8) = (500, 6.4, 1); 
    let five_hundred = x.0;  
} 

// arrays 
fn main() { 
    let a = [1, 2, 3, 4, 5]; 
    let months = [“jan”, “feb”, “mar”]; 
    let b: [i32; 5] = [1, 2, 3, 4, 5]; 
    let z = [3; 5]; 
    let first = a[0]; 
}

// Numeric Operations  

fn main() { 
    let sum = 15 + 2; 
    let difference = 15.3 - 2.2; 
    let multiplication = 2 * 20; 
    let division = 20 / 2; 
    let remainder = 21 %2 ; 
} 

// Slices  
fn main() { 
    let n1 = “example”.to_string(); 
    let c1 = &n1[4..7]; 
} 


fn main() { 
    let arr = [5, 7, 9, 11, 13]; 
    let slice = &arr[1..3]; 
    assert_eq!(slice, &[7, 9]); 
}


// Strings 
fn main() { 
    let s = “hello”; 
} 

// String type
fn main() { 
    let mut hello = String::from(“hello”); 
    hello.push('w'); 
    hello.push_str(“world!”); 
} 

// ‘to.string()’ 
fn main() { 
    let i = 5; 
    let five = String::from(“5”); 
    assert_eq!(five, i.to_string()); 
}

// Enumns

enum CacheType{ 
    LRU, 
    MRU, 
} 
let lru = CacheType::LRU; 
let mru = CacheType::MRU;  

struct Cache{ 
    level: String, 
    type: CacheType 
} 

// Control Flow
fn main() { 
    let i = 5; 
    if i > 3 { 
       println!(“condition met, i is greater than 3”); 

    } else { 
        println!(“condition was not met”); 
    } 
} 

// Control Flow 2
fn main() { 
    let a = 10; 
    if a % 4 == 0 { 
        println!(“a is divisible by 4”); 
    } else if a % 3 == 0 { 
        println!(“a is divisible by 3”); 
    } else if a % 2 == 0 { 
        println!(“a is divisible by 2”); 
    } else { 
        println!(“a is not divisible by 4,3, or 2”); 
    } 
} 


// Functions  
fn main() { 
    let a = plus_ten(); 
    println!(“the value of a is: {a}”); 
} 

fn plus_ten(a: i32) -> i32 { 
    a+10 
}


// Match Control Flow 

enum Web3{ 
    Defi, 
    NFT, 
    Game, 
    Metaverse 
} 
fn number_assign(web3: Web3) -> u8 { 
    match web3 { 
        Web3::Defi => 1, 
        Web3::NFT => 2, 
        Web3::Game => 3, 
        Web3::Metaverse => 4, 
    } 
} 

// Structs

struct Employee { 
    name: String, 
    assigned_id: u64, 
    email: String, 
    active: bool, 
} 

fn main () { 
    let emp1 = Employee { 
        name: String::from(“emplyee1”) 
        assigned_id: 1, 
        email: String::from(“employee@acme.com”) 
        active: true, 
    } 
} 

// Vectors

Let vector = vec![1, 2, 3]; 

Let vector: Vec<i32> = Vec::new( ); 

struct rectangle { 
    w: i8 
    h: i8 
} 

let mut v = vec![]; 
v.push(rectangle{w: 3, h: 4}); 
v.push(rectangle{w: 99, h: 42});


// Hashmaps

fn main() { 
    use std::collections::HashMap; 
    let mut rgb = HashMap::new(); 
    rgb.insert(String::from("Blue"), 10); 
    rgb.insert(String::from("Green"), 50); 
    rgb.insert(String::from(“Red”), 100); 
    for (key, value) in &rgb{ 
        println!(“{key}:{value}”); 
    } 
}


// Ownership and Borrowing
fn main() { 
    let example = String::from("hello");                      
}

fn main( ){ 
    let example = String::from(“hello”); 
    another_function(example); 
    println!(“{}”, example) 
} 
fn another_function(example String){ 
    println!(“{}”, example) 
} 

fn main() { 
    let s1 = String::from("hello"); 
    let len = calculate_length(&s1); 
    println!("The length of '{}' is {}.", s1, len); 
} 

fn calculate_length(s: &String) -> usize { 
    s.len() 
} 

// Crates, Modules, and Cargo

mod english { 
    mod greetings { 
    } 
    mod farewells { 
    } 
} 
use phrases::english::greetings; 
use phrases::english::farewells; 