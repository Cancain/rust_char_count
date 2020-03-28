use std::io;

fn main() {
    greeting();
    let input = take_input();
    let num_of_characters = get_num_of_characters(input);
    println!("you entered {} numbers excluding whitespace", num_of_characters - 1);
}

fn greeting() {
    println!("Please enter a string");
}

fn take_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("sorry, something was wrong with your input");
    input
}

fn get_num_of_characters(input: String) -> usize {
    let new_input = remove_whitespace(input);

    new_input.chars().count()
}

fn remove_whitespace(string: String) -> String {
    let mut new_string = String::from("");
    for char in string.chars() {
        if char != ' '{
            new_string.push(char)
        }
    }
    new_string
}
