use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
// use soloud::*;

use std::{
    io::{self, stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    println!("Hello, world!");
    let mut work_time = 25;
    let mut rest_time = 5;
    loop {
        print_menu();
        println!("Select an option:");
        let line = get_integer();
        match line {
            1 => {
                println!("Let's work!");
                start_pomodoro_stage(&work_time);
                play_alarm();
                println!("Let's rest a bit");
                start_pomodoro_stage(&rest_time);
                play_alarm();
            }
            2 => {
                change_time_values(&mut work_time, &mut rest_time);
                println!("W {}, R {}", work_time, rest_time);
            }
            3 => print_license(),
            4 => break,
            _ => println!("Please, select an available option"),
        }
    }
}

fn get_integer() -> u32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    return match buffer.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please, type a valid number");
            0
        }
    };
}

fn start_pomodoro_stage(time: &u32) {
    let time_on_seconds = time * 60;
    let mut stdout = stdout();
    for i in (0..=time_on_seconds).rev() {
        print!("\r{} seconds remaining...", i);
        stdout.flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}

fn play_alarm() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = io::BufReader::new(File::open("Audio/OnePieceGoldBellSoundEffect.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    // let mut sl = Soloud::default()?;

    // let mut wav = audio::Wav::default();

    // wav.load(&std::path::Path::new(
    //     "Audio/OnePieceGoldBellSoundEffect.mp3",
    // ))?;

    // sl.play(&wav);
    // while sl.voice_count() > 0 {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    // Ok(())
}

fn change_time_values(working_time: &mut u32, resting_time: &mut u32) {
    println!("Type the minutes you want to work with:");
    *working_time = get_integer();
    println!("Type the minutes you want to rest:");
    *resting_time = get_integer();
    println!("Values changed succesfully!");
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
