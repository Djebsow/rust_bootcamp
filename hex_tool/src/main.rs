use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};

#[derive(Parser, Debug)]
#[command(name = "hextool", about = "Read and write binary files in hexadecimal")]
struct Args {
    /// Target file
    #[arg(short, long)]
    file: String,

    /// Read mode (display hex)
    #[arg(short, long)]
    read: bool,

    /// Write mode (hex string to write)
    #[arg(short, long)]
    write: Option<String>,

    /// Offset in bytes (decimal or 0x hex)
    #[arg(short, long, default_value = "0")]
    offset: String,

    /// Number of bytes to read
    #[arg(short, long)]
    size: Option<usize>,
}

fn parse_offset(offset_str: &str) -> u64 {
    if offset_str.starts_with("0x") {
        u64::from_str_radix(&offset_str[2..], 16).expect("Invalid hex offset")
    } else {
        offset_str.parse::<u64>().expect("Invalid decimal offset")
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let offset = parse_offset(&args.offset);

    if args.read {
        // MODE LECTURE 
        let mut file = File::open(&args.file)?;
        file.seek(SeekFrom::Start(offset))?;

        let mut buffer = vec![0; args.size.unwrap_or(32)];
        let n = file.read(&mut buffer)?;
        let buffer = &buffer[..n];

        for (i, chunk) in buffer.chunks(16).enumerate() {
            let current_offset = offset + (i * 16) as u64;
            
            // Affichage Hexadécimal
            print!("{:08x}: ", current_offset);
            for byte in chunk { print!("{:02x} ", byte); }
            
            // Alignement si le dernier chunk est incomplet
            if chunk.len() < 16 {
                for _ in 0..(16 - chunk.len()) { print!("   "); }
            }

            // Affichage ASCII
            print!(" |");
            for &byte in chunk {
                if byte >= 32 && byte <= 126 { print!("{}", byte as char); }
                else { print!("."); }
            }
            println!("|");
        }
    } else if let Some(hex_to_write) = args.write {
        // MODE ÉCRITURE (avec création de fichier si absent)
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Permet de créer test.bin s'il n'existe pas
            .open(&args.file)?;

        file.seek(SeekFrom::Start(offset))?;

        // Conversion String Hex -> Vec<u8>
        let bytes = (0..hex_to_write.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_to_write[i..i + 2], 16).expect("Invalid hex string"))
            .collect::<Vec<u8>>();

        println!("Writing {} bytes at offset 0x{:08x}", bytes.len(), offset);
        file.write_all(&bytes)?;
        println!("Successfully written");
    }

    Ok(())
}