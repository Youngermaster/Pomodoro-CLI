fn main() {
    println!("Hello, world!");
    loop {
        print_menu();
        let mut line = String::new();
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("Hello , {}", line);
        println!("no of bytes read , {}", b1);
        match line {
            "1" => println!("One"),
            _ => println!("Ain't special"),
        }
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

let n = get_input().trim().parse::<i64>().unwrap();
fn print_menu() {
    println!(
        "
        |================================================================|
        |                         CLI Pomodoro                           |
        |================================================================|
        |                                                                |
        | Select an option:                                              |
        |                                                                |
        | 1. Start pomodoro                                              |
        | 2. Change time values                                          |
        | 3. Quit                                                        |
        | 4. LICENSE                                                     |
        |                                                                |
        |================================================================|

        "
    );
}

fn print_license() {
    println!(
        "
        MIT License

        Copyright (c) 2021 Juan Manuel Young Hoyos

        Permission is hereby granted, free of charge, to any person obtaining a copy
        of this software and associated documentation files (the \"Software\"), to deal
        in the Software without restriction, including without limitation the rights
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        copies of the Software, and to permit persons to whom the Software is
        furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all
        copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        SOFTWARE.
        "
    )
}
