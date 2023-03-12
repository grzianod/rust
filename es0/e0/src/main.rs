use std::env::args;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Result};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

///     Program to convert legacy C data in Rust
#[derive(Parser,  Debug)]
#[command(version,  long_about  =  None)]

struct  Args  {
///     File system path to exported data's file
#[arg(short,  long)]
file:  String,
}

#[derive(Debug, Deserialize)]
struct ValueStruct {
    _type: i32,
    val: f32,
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MValueStruct {
    _type: i32,
    val: [ f32; 10 ],
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MessageStruct {
    _type: i32,
    message: [u8; 21],
}

#[derive(Debug, Deserialize)]
enum ValueUnion {
    Value(ValueStruct),
    MValue(MValueStruct),
    Message(MessageStruct),
}

#[derive(Debug, Deserialize)]
struct CData {
    type_: i32,
    value_union: ValueUnion,
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("{}", args.file);
    let mut file = File::open(args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded :CData = bincode::deserialize(&buffer).unwrap();
    println!("Struttura letta dal file: {:?}", decoded);
    Ok(())
}