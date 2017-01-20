use std::fs::File;
use std::io::Read;
use std::error::Error;

type Contents = String;
type ErrorString = String;

pub fn read(path: String) -> Result<Contents, ErrorString> {
    let mut file = match File::open(path.clone()) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to open `{}`: {}", path, e.description())),
    };
    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => Ok(code),
        Err(e) => Err(format!("Unable to read `{}`: {}", path, e.description())),
    }
}
