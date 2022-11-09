use std::{
    io::{self, BufRead, Read, Stdin, Stdout, Write},
    thread,
    time::{Duration, Instant},
};

static mut FIBONACCI_COUNTER: u128 = 0;
static DEFAULT_NTH: u8 = 30;

fn main() {
    let mut stdin: Stdin = io::stdin();
    let mut stdout: Stdout = io::stdout();

    print!("Enter which Fibonacci Number to calculate (1-60): ");

    stdout.flush().unwrap();

    let fibonacci_nth: u8 = match stdin.lock().lines().next() {
        None => {
            println!("Fibonacci Number is not provided, defaulting to {}", DEFAULT_NTH);
            DEFAULT_NTH
        }
        Some(result) => match result {
            Ok(string) => match u8::from_str_radix(&string, 10) {
                Ok(number) => match number {
                    1..=60 => number,
                    _ => {
                        println!("Provided number is outside of 1-60 range, defaulting to {}", DEFAULT_NTH);
                        DEFAULT_NTH
                    }
                },
                Err(error) => {
                    println!("Error processing number from the given input: ({}), defaulting to {}", error, DEFAULT_NTH);
                    DEFAULT_NTH
                }
            },
            Err(error) => {
                println!("Something has gone terribly wrong: ({}), defaulting to {}", error, DEFAULT_NTH);
                DEFAULT_NTH
            }
        },
    };

    let ordinal_suffix: &str = get_ordinal_suffix(fibonacci_nth);
    let before_iter_time: Instant = Instant::now();
    let fibonacci_nth_value_iter: u128 = get_fibonacci_iterative(fibonacci_nth);
    let after_iter_time: Duration = before_iter_time.elapsed();
    println!(
        "The {}{} fibonacci number is {}, calculated iteratively in {} seconds",
        fibonacci_nth,
        ordinal_suffix,
        fibonacci_nth_value_iter,
        after_iter_time.as_secs_f32()
    );

    unsafe {
        thread::spawn(move || loop {
            let counter_thread: u128 = FIBONACCI_COUNTER;
            if counter_thread <= fibonacci_nth_value_iter {
                progress_bar(
                    counter_thread,
                    fibonacci_nth_value_iter,
                    50,
                    "calculating fibonacci recursively",
                );
                if counter_thread == fibonacci_nth_value_iter {
                    break;
                }
            }
        });
        thread::sleep(Duration::from_millis(5));

        let before_rec_time: Instant = Instant::now();
        let fibonacci_nth_value_rec: u128 = get_fibonacci_recursive(fibonacci_nth);
        let after_rec_time: Duration = before_rec_time.elapsed();
        println!(
            "The {}{} fibonacci number is {}, calculated recursively in {} seconds",
            fibonacci_nth,
            ordinal_suffix,
            fibonacci_nth_value_rec,
            after_rec_time.as_secs_f32()
        );

        let diff_time: f32 = after_rec_time.as_secs_f32() / after_iter_time.as_secs_f32();
        if diff_time > 1.0 {
            println!(
                "The recursive implementation was {:.2} times slower, than iterative!",
                diff_time
            );
        }

        stdout.write(b"Press Enter to exit...").unwrap();
        stdout.flush().unwrap();
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}

fn get_fibonacci_iterative(num_iter: u8) -> u128 {
    let mut first_int: u128 = 0;
    let mut second_int: u128 = 1;
    let mut tmp: u128;

    for _ in 0..num_iter - 1 {
        tmp = first_int + second_int;
        first_int = second_int;
        second_int = tmp;
    }

    return second_int;
}

fn get_fibonacci_recursive(num_iter: u8) -> u128 {
    unsafe {
        match num_iter {
            0 => return 0,
            1 => {
                FIBONACCI_COUNTER += 1;
                return 1;
            }
            _ => {
                return get_fibonacci_recursive(num_iter - 1)
                    + get_fibonacci_recursive(num_iter - 2)
            }
        }
    }
}

fn get_ordinal_suffix(int: u8) -> &'static str {
    let suffix: &str = match int {
        4..=20 => "th",
        _ => match int % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    return suffix;
}

fn progress_bar(current: u128, maximum: u128, segments: usize, info: &str) -> () {
    let percentage: f64 = current as f64 / maximum as f64;

    let full_segments: usize = (percentage * segments as f64).floor() as usize;

    let bar: String = str::repeat("#", full_segments) + &str::repeat("-", segments - full_segments);

    let state: char = {
        if current == maximum {
            '✓'
        } else {
            match current % 4 {
                0 => '/',
                1 => '-',
                2 => '\\',
                3 => '|',
                _ => '?',
            }
        }
    };

    print!("\r[{}] {:.2}% {} / {} {} {}", bar, percentage * 100 as f64, current, maximum, state, info);

    if current == maximum {
        println!()
    }
}
