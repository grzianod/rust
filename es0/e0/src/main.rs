use std::env::args;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Result};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::mem;

///     Program to convert legacy C data in Rust
#[derive(Parser,  Debug)]
#[command(version,  long_about  =  None)]

struct  Args  {
///     File system path to exported data's file
#[arg(short,  long)]
file:  String,
}

#[derive(Debug, Deserialize)]
struct ValueStruct {    //size: 16 bytes
    _type: i32,
    val: f32,
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MValueStruct {   //size: 56 bytes
    _type: i32,
    val: [ f32; 10 ],
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MessageStruct {  //size: 28 bytes
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
struct CData {  //size: 64 bytes
    type_: i32,
    value_union: ValueUnion,
}

impl CData {
    fn from_file(mut file: File) -> Result<Vec<CData>> {
        let mut buffer = [0u8; 64];
        let mut result = Vec::with_capacity(100);
        for _ in 0..100 {
            file.read_exact(&mut buffer)?;
            let cdata: CData = bincode::deserialize(&buffer).unwrap();
            result.push(cdata);
        }
        Ok(result)
    }

    fn print_data(data: Vec<CData>) {
        for i in 0..100 {
            println!("{:?}", data[i]);
        }
    }
}

fn main() -> Result<()> {
    let size = mem::size_of::<CData>();
    println!("sizeof(CData) = {}", size);
    let args = Args::parse();
    let mut file = File::open(args.file)?;
    match CData::from_file(file) {
        Ok(data) => {
            CData::print_data(data);
        }
        Err(error) => {
            println!("Error reading data from file\n");
        }
    }
    Ok(())
}