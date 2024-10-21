use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;
fn main() 
{   
    let mut _passwords: HashMap<String, String> = HashMap::new();
    let mut _password: String = get_password();
    let mut not_auth: bool = true;
    load(&mut _passwords);

    if _password == ""
    {
        _password = set_password();
        not_auth = false;
    }

    while not_auth
    {
        not_auth = check_auth(&mut _password);
    }
    let mut running = true;

    while running 
    {
        match menu().as_str() {
            "1" => add_password(&mut _passwords),
            "2" => view_passwords(&mut _passwords),
            "3" => delete_password(&mut _passwords),
            "4" => 
            {
            save(&mut _passwords, &mut _password);
            running = false;},
            _ => println!("Invalid selection"),
        }
    }
}

fn menu() -> String {

    println!("Menu");
    println!("1. Add Password");
    println!("2. View Passwords");
    println!("3. Delete Password");
    println!("4. Exit");
    println!("\n Make a selection: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let answer = input.trim().to_string();

    return answer;   
}

fn set_password() -> String 
{
    println!("Create a password: ");
    let mut password: String = String::new();
    std::io::stdin().read_line(&mut password).expect("could not read line");
    return password;
}

fn get_password() -> String 
{
    let mut _pas: String;
    let filename: &str = "passwords.txt";
    let path = Path::new(filename);
    if path.exists() 
    {
        let mut _file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        _file.read_to_string(&mut contents).expect("Failed to read file");
        for line in contents.lines() {
            _pas = line.to_string();
            return _pas;
        }
    } 
    return String::new();
    
}

fn load(passwords: &mut HashMap<String, String>) 
{
    let filename: &str = "passwords.txt";
    let path = Path::new(filename);
    if path.exists() 
    {
        let mut _file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        _file.read_to_string(&mut contents).expect("Failed to read file");
        let mut line_counter = 1;
        for line in contents.lines() 
        {
            if line_counter == 1 {
                continue;
            } else {
             let parts: Vec<&str> = line.split(":").collect();
            let website = parts[0];
            let password = parts[1];
            passwords.insert(website.to_string(), password.to_string());
            line_counter += 1;}
        }
    } else
    {
    let mut _file = File::create(path).expect("Failed to create file");
    }
}

fn save(passwords: &mut HashMap<String, String>, password: &mut String) {
    let filename: &str = "passwords.txt";
    let path = Path::new(filename);
    let mut _file = OpenOptions::new().write(true).open(path).unwrap();
    writeln!(_file, "{}",password).expect("Cannot write to file");
    for (website, password) in passwords.iter()
    {
        let line: String = format!("{website}{password}");
        _file.write(line.as_bytes()).expect("Cannot write to file");
    }
    println!("Saved Successfully")
}

fn add_password(passwords: &mut HashMap<String, String>) {
    println!("Enter website: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    println!("Enter password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).expect("Failed to read line");
    passwords.insert(username, password);
}

fn view_passwords(passwords:&mut HashMap<String, String>) {
    if passwords.is_empty() {
        println!("No passwords found");
    }
    else {
        for (website, password) in passwords {
            println!("{}: {}", website, password);
        }
    }
}

fn delete_password(passwords:&mut HashMap<String, String>) {
    println!("Enter website to delete: ");
    let mut website = String::new();
    std::io::stdin().read_line(&mut website).expect("Failed to read line");
    if passwords.contains_key(website.as_str()) {
        passwords.remove(&website);
    }
}

fn check_auth(password: &mut String) -> bool {
    println!("Enter Password");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read line");
    if input == password.as_str(){
        return false;
    }
    else {
        print!("Incorrect Password");
        return true;}
}