use std::collections::HashSet;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::ops;
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Clone)]
struct IntCode {
    pub memory: Vec<usize>,
}
impl FromStr for IntCode {
    type Err = <usize as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the line into integers
        let memory = s.split(',').map(|i| i.parse()).collect::<Result<_, _>>()?;

        Ok(IntCode { memory })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim_end();

    let program: IntCode = line.parse()?;

    let mut memory = program.memory;

    memory[1] = 12;
    memory[2] = 2;

    let mut written_to = HashSet::new();
    let mut pos = 0;
    println!("CODE: ");
    while pos < memory.len() {
        match memory[pos] {
            1 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                written_to.insert(c);
                println!("{:02X}: ADD {:02X} {:02X} {:02X}", pos, a, b, c);
                pos += 4
            }
            2 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                written_to.insert(c);
                println!("{:02X}: MUL {:02X} {:02X} {:02X}", pos, a, b, c);
                pos += 4
            }
            99 => {
                println!("{:02X}: HALT", pos);
                pos += 1;
                break;
            }
            d => {
                println!("{:02X}: UNKNOWN: {:02X}", pos, d);
                pos += 1;
            }
        }
    }
    println!();
    println!("DATA: ");
    for pos in pos..memory.len() {
        println!("{:02X}: {}", pos, memory[pos]);
    }

    let mut pos = 0;
    println!("CODE: ");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    while pos < memory.len() {
        match memory[pos] {
            1 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                write_address(pos, pos, &stdout, &written_to);
                print!(": ADD ");
                write_address(pos + 1, a, &stdout, &written_to);
                print!(" ");
                write_address(pos + 2, b, &stdout, &written_to);
                print!(" ");
                write_address(pos + 3, c, &stdout, &written_to);
                println!();
                pos += 4
            }
            2 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                write_address(pos, pos, &stdout, &written_to);
                print!(": MUL ");
                write_address(pos + 1, a, &stdout, &written_to);
                print!(" ");
                write_address(pos + 2, b, &stdout, &written_to);
                print!(" ");
                write_address(pos + 3, c, &stdout, &written_to);
                println!();
                pos += 4
            }
            99 => {
                println!("{:02X}: HALT", pos);
                pos += 1;
                break;
            }
            d => {
                println!("{:02X}: UNKNOWN: {:02X}", pos, d);
                pos += 1;
            }
        }
    }
    println!();
    println!("DATA: ");
    for pos in pos..memory.len() {
        write_address(pos, pos, &stdout, &written_to);
        print!(": ");
        write_address(pos + 1, memory[pos], &stdout, &written_to);
        println!();
    }

    let mut pos = 0;
    println!("graph uwu {{");
    println!("{{");
    for k in 0..memory.len() {
        println!(r#"{} [label="{:02X}"]"#, k, k);
    }
    println!("}}");
    while pos < memory.len() {
        match memory[pos] {
            1 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                println!("{} -> {}", a, pos);
                println!("{} -> {}", b, pos);
                println!("{} -> {}", pos, c);
                pos += 4
            }
            2 => {
                let a = memory[pos + 1];
                let b = memory[pos + 2];
                let c = memory[pos + 3];
                println!("{} -> {}", a, pos);
                println!("{} -> {}", b, pos);
                println!("{} -> {}", pos, c);
                pos += 4
            }
            99 => {
                //println!("{:02X}: HALT", pos);
                pos += 1;
                break;
            }
            d => {
                //println!("{:02X}: UNKNOWN: {:02X}", pos, d);
                pos += 1;
            }
        }
    }
    //println!();
    //println!("DATA: ");
    for pos in pos..memory.len() {
        //println!("{:02X}: {}", pos, memory[pos]);
    }
    println!("}}");

    Ok(())
}

fn write_address(pos: usize, addr: usize, stdout: &StandardStream, written: &HashSet<usize>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if written.contains(&pos) {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
    }
    write!(&mut stdout, "{:02X}", addr);
    stdout.reset();
}

enum MemoryState {
    Constant(usize),
}
