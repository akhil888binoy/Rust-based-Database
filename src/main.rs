use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, stdin};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs;
use std::fs::OpenOptions;



fn set(key : &String )->String{
    let contents = fs::read_to_string("foo.txt").expect("Should have been able to read the file");
    let mut storage : HashMap<String , String> = serde_json::from_str(&contents).unwrap();
    let input = getinput("Enter the data");
    storage.insert(key.to_string(), input);

    let mut file = OpenOptions::new().write(true).truncate(true).open("foo.txt").unwrap();
    serde_json::to_writer(&file, &storage).unwrap();
    return "Setted data".to_string();

}

fn get()->String{
    let key = getinput("Give input of key");
    let contents = fs::read_to_string("foo.txt").expect("Should have been able to read the file");
    let storage : HashMap<String , String> = serde_json::from_str(&contents).unwrap();
    match storage.get(&key){
        Some(value)=> value.to_string() ,
        None=>"Key not found".to_string()
    }
}


fn delete()->String{
    let key = getinput("Give input of key");
    let contents = fs::read_to_string("foo.txt").expect("Should have been able to read the file");
    let mut storage : HashMap<String , String> = serde_json::from_str(&contents).unwrap();
   let result =  match storage.remove(&key){
        Some(value)=> {
            let mut file = OpenOptions::new().write(true).truncate(true).open("foo.txt").unwrap();
            serde_json::to_writer(&file, &storage).unwrap();
            return "Key deleted successfully".to_string();
        },
        None=>"Key not found".to_string()
    };
    return result;
}


fn getinput(p:&str)->String{
    println!("{}",p);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn getHash()->String{
    let mut hasher = DefaultHasher::new();
    7920.hash(&mut hasher);
    hasher.finish().to_string()
}


fn main() {
    let mut storage : HashMap<String,String> = HashMap::new();
    let hash = getHash();
    loop{
        let choice : u32 = getinput("Enter the choice").parse().unwrap();
        let result = match choice {
            1=>set(&hash),
            2=>get(),
            3=>delete(),
            _=> "Enter a valid choice".to_string()
        };
        println!("{}", result);
    }
    
}
