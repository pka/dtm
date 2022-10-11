use dtm::DTM;
use std::path::Path;
use std::{env, process};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: convert <in.png>");
        process::exit(2);
    }

    let filein = &args[1];
    let source = image::open(filein).unwrap();

    let descriptor1 = DTM {
        pixel_size: 10, // TODO: depends on input file
        channel_count: 1,
        width: source.width() as usize,
        height: source.height() as usize,
    };
    let data1 = source.as_luma_alpha16().unwrap().to_vec();

    let fileout = Path::new(filein).with_extension("dtm");
    descriptor1.encode_file(&fileout, &data1).unwrap();
}