#![no_main]
use libfuzzer_sys::fuzz_target;
use crossterm::style::Colored;

fuzz_target!(|input: String| {
    let _ = Colored::parse_ansi(&input);
});