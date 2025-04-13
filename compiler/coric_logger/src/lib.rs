use std::ops::Range;

pub struct Logger {
    src: String,
    filename: String,
}

impl Logger {
    pub fn new(src: String, filename: String) -> Self {
        Logger { src, filename }
    }

    pub fn error(self, message: String, range: Range<usize>) {
        println!("\x1b[31merror\x1b[0m in {}", self.filename,);
        println!("{}", self.src[range].to_string());
        println!("{}", message);
        Self::exit()
    }

    fn exit() {
        std::process::exit(1);
    }
}
