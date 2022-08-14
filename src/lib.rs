pub struct Config {
    pub xml: String,
    pub filename: String,
    pub output: Processing,
}

pub enum Processing {
    Table,
    Summary,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let xml = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get xml file"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a file to parse"),
        };
        let output = match args.next() {
            Some(arg) if arg == "table" => Processing::Table,
            Some(arg) if arg == "summary" => Processing::Summary,
            Some(_) => return Err("did not specify result output, options 'table' and 'summary'"),
            None => return Err("did not specify result output, options 'table' and 'summary'"),
        };
        Ok(Config {
            xml,
            filename,
            output,
        })
    }
}
