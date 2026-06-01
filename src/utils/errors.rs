pub struct Error {
    message: String,
    line: Option<usize>,
    reason: Option<ErrorReason>,
}

pub struct ErrorReason {
    error_type: Reason,
    data: Option<Vec<String>>,
}

pub enum Reason {
    Parse,
    AST,
    Lexer,
    Interpreter,
    Utils,
    Compile,
    Runtime,
}

impl Error {
    pub fn init(message: String, line: Option<usize>, reason: Option<ErrorReason>) -> Self {
        Self {
            message,
            line,
            reason,
        }
    }

    pub fn print_error(&self) {
        match &self.line {
            Some(l) => println!("[{}) Error: {}]", l, self.message),
            None => println!("[Error: {}]", self.message),
        }
        match &self.reason {
            Some(r) => match &r.data {
                Some(d) => {
                    println!("[{}]", r.get_type_string());
                    for l in d {
                        println!("{}", l);
                    }
                }
                _ => println!("[{}]", r.get_type_string()),
            },

            None => {}
        }
        std::process::exit(1);
    }
}

impl ErrorReason {
    pub fn init(error_type: Reason, data: Option<Vec<String>>) -> Self {
        Self { error_type, data }
    }

    fn get_type_string(&self) -> String {
        match &self.error_type {
            Reason::Parse => "Parse Error",
            Reason::AST => "AST Error",
            Reason::Lexer => "Lexer Error",
            Reason::Interpreter => "Interpreter Error",
            Reason::Utils => "Utils Error",
            Reason::Compile => "Compile Error",
            Reason::Runtime => "Runtime Error",
        }
        .to_string()
    }

    fn print_error_reason(&self) {
        match &self.data {
            Some(d) => {
                println!("[{}]:", &self.get_type_string());
                for line in d.iter() {
                    println!("\t{line}");
                }
            }
            None => println!("[{}]", &self.get_type_string()),
        }
    }
}
