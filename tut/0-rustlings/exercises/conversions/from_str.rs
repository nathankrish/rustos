// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let words: Vec<&str> = s.split(",").collect();
        if words.len() < 2 {
            return Err(String::from("not enough data"));
        }
        let name = words[0].to_string();
        if let Ok(age) = words[1].parse::<usize>() {
            return Ok(Person {
                name,
                age
            });
        } else {
            return Err(String::from("couldn't parse age"));
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}