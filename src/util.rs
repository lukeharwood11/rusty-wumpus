use std::str::FromStr;
#[derive(Debug)]
pub struct Point(pub usize, pub usize);

pub struct Command {

}

impl Command {

}

enum ReadError {
    NoValue,
    TypeError
}

/// Given a buffer `buff` remove the first string and parse it as the given type
/// Returns a `Result<T, ReadError>`
/// ReadError::NoValue when the buffer is empty
/// ReadError::TypeError when the value cannot be parsed to the given type
/// Note: values are removed regardless of success from the pop
fn read_buffer_as<T: FromStr>(buff: &mut Vec<&str>) -> Result<T, ReadError> {
    if buff.len() > 0 {
        let string = buff.remove(0);
        string.parse::<T>().map_err(|err| ReadError::TypeError)
    } else {
        Err(ReadError::NoValue)
    }
}

/// prompts the user for instructions, returns the text from the user
pub fn get_instruction() -> String {
    let mut line = String::new();
    print!(">>> ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    line
}