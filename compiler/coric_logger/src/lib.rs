use std::ops::Range;

pub struct Logger {
    src: String,
    lines: Vec<String>,
    lines_start: Vec<usize>,
    last_line: usize,
    filename: String,
}

impl Logger {
    pub fn new(src: String, filename: String) -> Self {
        let lines_iter = src.split("\n");
        let last_line = lines_iter.clone().collect::<Vec<&str>>().len();
        let mut pos = 0;
        let mut lines = Vec::new();
        let mut lines_start = Vec::new();

        for line in lines_iter {
            lines.push(line.to_string());
            lines_start.push(pos);
            pos += line.len() + 1;
        }

        Logger {
            src,
            lines,
            lines_start,
            last_line,
            filename,
        }
    }

    fn line(&self, start: usize) -> Option<usize> {
        for i in (0..self.lines_start.len()).rev() {
            if self.lines_start[i] <= start {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn message(&self, message: &str) {
        println!("\x1b[1;32m{}\x1b[0m", message);
    }

    pub fn error(&self, message: &str, range: Range<usize>) {
        let mut line_start = self.line(range.start).unwrap();
        let mut line_end = self.line(range.end).unwrap();
        let olines = line_start;

        println!("\x1b[31merror\x1b[0m: {}", message);

        if line_start != 1 {
            line_start -= 1;
        }

        if line_end != self.last_line {
            line_end += 1;
        }

        let mut elines = String::new();
        for line in line_start - 1..line_end {
            elines.push_str(&self.lines[line]);
            elines.push('\n');
        }

        elines = elines.replace(
            &self.src[range.clone()].to_string(),
            &format!("\x1B[4;31m{}\x1B[0m", self.src[range.clone()].to_string()),
        );
        elines = elines.trim().to_string();

        println!("~~~~~~~~~~~~~~~~~~~~~~~~");

        println!(
            "--> {}:{}:{}",
            self.filename,
            olines,
            range.start - self.lines_start[olines - 1] + 1
        );

        println!("{}~~~~~~~~~~~~~~~~~~~~~~~~", elines);
    }

    pub fn exit() -> ! {
        std::process::exit(1);
    }
}
