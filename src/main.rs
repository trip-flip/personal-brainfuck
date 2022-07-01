use std::env::args;

mod bf {
    pub fn from_input(input: &[u8]) {
        let mut data_index = 0;
        let mut input_index = 0;
        let mut data: Vec<u8> = vec![0];
        let mut loop_i = Vec::new();
        let mut skip = false;
        let mut last_len = 0;

        while input_index < input.len() {
            let c = input[input_index];

            if !skip {
                match c as char {
                    '>' => {
                        data_index += 1;
                        if data_index >= data.len() {
                            data.resize(data_index + 1, 0);
                        }
                    },
                    '<' => data_index -= 1,
                    '+' => data[data_index] += 1,
                    '-' => data[data_index] -= 1,
                    '.' => print!("{}", data[data_index] as char),
                    ',' => (),
                    _ => ()
                }
            }

            match c as char {
                '[' => {
                    loop_i.push(input_index);
                    if data[data_index] != 0 { // Predicate true
                        last_len = loop_i.len();
                    } else {
                        skip = true;
                    }
                },
                ']' => {
                    if skip {
                        loop_i.pop();
                        if last_len == loop_i.len() {
                            skip = false;
                        }
                    } else {
                        if data[data_index] == 0 {
                            loop_i.pop();
                        } else {
                            input_index = *loop_i.last().unwrap();
                        }
                    }
                },
               _ => ()
            }
            input_index += 1;
        }
    }

    // fn err<T>(m: &str) -> Result<T, String> {
    //     Err(m.to_string())
    // }
}

fn main() {
    let arg = args().nth(1);
    if arg.is_none() {
        return;
    }

    let path = arg.unwrap();
    let mut input = std::fs::read_to_string(path).unwrap();
    input.retain(|c| !c.is_whitespace());
    bf::from_input(input.as_bytes());
}
