pub enum InputTypes {
    Example,
    MyInput,
}

impl InputTypes {
    pub fn to_file_name(&self) -> &str {
        match self {
            InputTypes::Example => "example.txt",
            InputTypes::MyInput => "input.txt",
        }
    }
}
