use simple_error::SimpleError;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
    pub fn from_args(args: &[String]) -> Result<Config, SimpleError> {
        if args.len() != 3 {
            return Result::Err(SimpleError::new("Expected 2 arguments"));
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Result::Ok(Self::new(query, filename))
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn query(&self) -> &String {
        &self.query
    }
}
