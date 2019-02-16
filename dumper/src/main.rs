extern crate sdl2;
extern crate byteorder;

use std::env;
use std::error::Error;
use std::path::Path;
use std::io::Read;
use std::io::Seek;
use std::fs::File;
use std::io::SeekFrom;
use sdl2::image::SaveSurface;
use byteorder::{ByteOrder, LittleEndian};

fn dump_sprite<P: AsRef<Path>>(archive: P, offset: u32, name: String, size: u32) {
    use std::io::Write;

    let mut rwop = sdl2::rwops::RWops::from_file(archive, "r").unwrap();
    rwop.seek(SeekFrom::Start(offset as u64)).unwrap();
    if name.ends_with(".LBM") {
        match sdl2::image::ImageRWops::load_lbm(&rwop) {
            Ok(surface) => {
                let mut new_name = String::from(name);
                new_name.push_str(".PNG");
                println!("Dumping: {}", new_name);
                surface.save(Path::new("target").join(&new_name));
            },
            Err(error) => println!("Dumping failed: {}", error),
        }
    } else {
        let mut buffer = vec![0; size as usize];
        match rwop.read_exact(&mut buffer) {
            Ok(()) => {
                let filename = String::from(name);
                println!("Dumping: {}", &filename);

                let path = Path::new("target").join(&filename);
                match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}",
                                       path.display(),
                                       why.description()),
                    Ok(mut file) => file.write_all(&mut buffer),
                };
            },
            Err(error) => println!("Dumping failed: {}", error),
        }
    }
}
    
fn main() {

    let args: Vec<_> = env::args().collect();
    let path: &Path = Path::new(&args[1]);

    let mut rwop = sdl2::rwops::RWops::from_file(path, "r").unwrap();

    //let mut buf = [1];
    let mut buf = vec![0u8; 1];
    rwop.read_exact(&mut buf).unwrap();

    println!("Number of files: {:?}", buf[0]);

    rwop.seek(SeekFrom::Current(1)).unwrap();

    for i in 0..buf[0] {
        println!("File #{}", i);
        let mut name = String::new();
        {
            let reference = rwop.by_ref();
            reference.take(12).read_to_string(&mut name);
        }
        // Trim nul bytes from filenames with < 12 characters
        let filename = name.trim_right_matches(char::from(0)).to_string();
        println!("- Filename: {}", filename);
        rwop.seek(SeekFrom::Current(2)).unwrap();
        let mut buffer = [0; 4];
        rwop.read_exact(&mut buffer).unwrap();
        let mut size = LittleEndian::read_u32(&buffer);
        println!("- Size: {:?}", size);
        rwop.read_exact(&mut buffer).unwrap();
        let mut offset = LittleEndian::read_u32(&buffer);
        println!("- Offset: {:?}", offset);
        
        dump_sprite(path, offset, filename, size);
    }

}
