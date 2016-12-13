use std::io;
use std::num::ParseIntError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ArchiveParseError {
            description("Illegal archive format")
            display("Illegal archive format")
        }
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
            display("{}", err)
        }
        Parse(err: ParseIntError) {
            from()
            cause(err)
            description(err.description())
            display("{}", err)
        }
    }
}
