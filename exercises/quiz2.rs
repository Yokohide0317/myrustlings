// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}



mod my_module {
    use super::Command as Command;

    fn Uppercase(input: &str) -> String {
        let output = input.to_uppercase();
        return output;
    }

    fn Trim(input: &str) -> String {
        let output = input.trim().to_string();
        return output;
    }

    fn Append(input: &str, i: &usize) -> String {
        let mut output = input.to_string();
        for _ in 1..*i+1 {
            output += "bar";
        }
        return output;
    }
    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let result: String = match command {
                Command::Uppercase => Uppercase(string),
                Command::Trim => Trim(string),
                Command::Append(i) => Append(string, i),
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
