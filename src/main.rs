use std::env::args;

mod bf {
    use std::num::Wrapping;

    pub fn parse(input: &[u8]) {
        let mut data_index = 0;
        let mut input_index = 0;
        let mut data: Vec<Wrapping<u8>> = vec![Wrapping(0)];
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
                            data.resize(data_index + 1, Wrapping(0));
                        }
                    },
                    '<' => data_index -= 1,
                    '+' => data[data_index] += 1,
                    '-' => data[data_index] -= 1,
                    '.' => print!("{}", data[data_index].0 as char),
                    ',' => (),
                    _ => ()
                }
            }

            match c as char {
                '[' => {
                    loop_i.push(input_index);
                    if data[data_index].0 != 0 { // Predicate true
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
                        if data[data_index].0 == 0 {
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
    let file_string = args()
        .nth(1)
        .map(|path| {
            std::fs::read_to_string(path).ok()
        }
    ).flatten();

    if file_string.is_none() {
        println!("No file with the provided path. Exitting....");
        return;
    }
    let mut input= file_string.unwrap();
    input.retain(|c| !c.is_whitespace());
    bf::parse(input.as_bytes());
}
