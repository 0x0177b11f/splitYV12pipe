extern crate lz4;

use std::env;
use std::fs::File;
use std::io;
use std::io::Result;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: &Vec<String> = &mut env::args().collect();

    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut frame: u32 = 0;
    let mut out_file: &str = "";

    let mut index: usize = 0;
    for argument in args {
        let arg: &str = &argument;
        let args_len: usize = *(&args.len());

        match arg {
            "-w" => {
                if index < (args_len - 1) {
                    width = args[index + 1].parse::<u32>().unwrap();;
                }
            }
            "-h" => {
                if index < (args_len - 1) {
                    height = args[index + 1].parse::<u32>().unwrap();;
                }
            }
            "-frame" => {
                if index < (args_len - 1) {
                    frame = args[index + 1].parse::<u32>().unwrap();
                }
            }
            "-o" => {
                if index < (args_len - 1) {
                    out_file = &args[index + 1];
                }
            }
            _ => {}
        }
        index += 1;
    }

    if width == 0 || height == 0 || frame == 0 || out_file == "" {
        println!("Usage of : splityv12pipe -w <width> -h <height> -frame <number> -o <filename>");
        return;
    }

    let max_bytes_len: usize = (frame * (width * height + width * height / 2)) as usize;
    compress(out_file.to_string(), max_bytes_len).unwrap();
}

fn compress(dst: String, maxlen: usize) -> Result<()> {
    let mut count: i32 = 0;
    let mut out: String = format!("{}_{}", dst, count.to_string());

    let mut fi = io::stdin();
    let mut buffer: [u8; 1024] = [0; 1024];

    let mut log_len: usize = 0;
    let mut fo = try!(lz4::EncoderBuilder::new().build(try!(File::create(&Path::new(&out)))));

    loop {
        let len = try!(fi.read(&mut buffer));
        if len == 0 {
            break;
        }
        if log_len + len >= maxlen {
            let mut l = 0;
            if (log_len + len) - maxlen > 0 {
                l = maxlen - log_len;
                try!(fo.write_all(&buffer[0..l]));
            }
            fo.finish();

            count += 1;
            out = format!("{}{}", dst, count.to_string());
            fo = try!(lz4::EncoderBuilder::new().build(try!(File::create(&Path::new(&out)))));

            try!(fo.write_all(&buffer[l..len]));
            log_len = 0;

            continue;
        }
        try!(fo.write_all(&buffer[0..len]));
        log_len += len;
    }
    match fo.finish() {
        (_, result) => result,
    }
}
