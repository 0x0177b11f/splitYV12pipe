extern crate lz4;

use std::env;
use std::fs::File;
use std::io;
use std::io::Result;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use lz4::Encoder;

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
    let count: i32 = 0;
    let file_name: String = format!("{}_{}", dst, count.to_string());
    let path: &Path = Path::new(&file_name);

    let mut fi = io::stdin();
    let mut fo = try!(lz4::EncoderBuilder::new().build(try!(File::create(path))));

    copy(&mut fi,
         &mut fo,
         &dst,
         maxlen,
         count + 1,
         &move || {
             println!("{}", path.display());
             return Ok(());
         })
        .unwrap();
    fo.finish();

    Ok(())
}

fn copy(fi: &mut Read,
        fo: &mut Encoder<std::fs::File>,
        dst: &str,
        maxlen: usize,
        count: i32,
        done: &Fn() -> Result<()>)
        -> Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut write_len: usize = 0;

    loop {
        let len = try!(fi.read(&mut buffer));
        if len == 0 {
            done().unwrap();
            break;
        }
        if write_len + len >= maxlen {
            let mut l = 0;
            if (write_len + len) - maxlen > 0 {
                l = maxlen - write_len;
                try!(fo.write_all(&buffer[0..l]));
            }
            done().unwrap();

            let file_name: &str = &format!("{}_{}", dst, count.to_string());
            let path = Path::new(file_name);
            let mut new_fo = try!(lz4::EncoderBuilder::new().build(try!(File::create(path))));
            try!(new_fo.write_all(&buffer[l..len]));

            copy(fi,
                 &mut new_fo,
                 dst,
                 maxlen,
                 count + 1,
                 &move || {
                     println!("{}", path.display());
                     return Ok(());
                 })
                .unwrap();

            new_fo.finish();

            break;
        }
        try!(fo.write_all(&buffer[0..len]));
        write_len += len;
    }
    Ok(())
}
