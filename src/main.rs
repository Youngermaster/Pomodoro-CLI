fn main() {
    println!("Hello, world!");
    let mut work_time = 25;
    let mut rest_time = 25;
    loop {
        print_menu();
        print!("Select an option: ");
        let line = get_integer();
        match line {
            1 => println!("One"),
            2 => change_time_values(&mut work_time, &mut rest_time),
            3 => print_license(),
            4 => break,
            _ => println!("Ain't special"),
        }
    }
}

fn get_integer() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let n = buffer.trim().parse::<i64>().unwrap();
    n
}

fn change_time_values(working_time: &mut i32, resting_time: &mut i32) {
    *working_time = 1;
    *resting_time = 1;
}

fn print_menu() {
    println!(
    "
    |=========================|
    |       Pomodoro CLI      |
    |=========================|
    |                         |
    | Select an option:       |
    |                         |
    | 1. Start pomodoro       |
    | 2. Change time values   |
    | 3. LICENSE              |
    | 4. Quit                 |
    |                         |
    |=========================|
    "
    );
}

fn print_license() {
    println!(
        "
        MIT License

        Copyright (c) 2222 Juan Manuel Young Hoyos

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
