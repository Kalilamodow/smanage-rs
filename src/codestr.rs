pub struct Code {
    value: String,
    indentation: usize,
}

impl Code {
    pub fn new(initial_capacity: usize) -> Code {
        Code {
            value: String::with_capacity(initial_capacity),
            indentation: 0,
        }
    }

    pub fn done(self) -> String {
        self.value
    }

    pub fn newline(&mut self) {
        self.value.push('\n');
    }

    pub fn add_line(&mut self, val: &str) {
        self.push_indentation();
        self.value.push_str(val);
        self.newline();
    }

    pub fn indent(&mut self) {
        self.indentation += 1;
    }

    pub fn outdent(&mut self) {
        self.indentation -= 1;
    }

    fn push_indentation(&mut self) {
        self.value.push_str(&"\t".repeat(self.indentation));
    }
}

impl std::fmt::Write for Code {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.value.push_str(s);
        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let s = std::fmt::format(args);
        self.add_line(&s);
        Ok(())
    }
}
