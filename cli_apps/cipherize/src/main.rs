use structopt::StructOpt;
use std::fs::File;
use std::io::{Write,stdout};


// Structs
#[derive(Debug,StructOpt)]
#[structopt(name = "cipherize", about="A simple CLI for cryptography and ciphers")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(short = "o", long = "output-file", help = "Output to a text file")]
    output_file: Option<String>,
}


// Enums: For our Subcommand
#[derive(Debug,StructOpt)]
enum Command {
    #[structopt(name="encode",about="Encode a message using a cipher")]
    Encode {
        #[structopt(short,long, help="The cipher to use (e.g. rot13,reverse,caesar")]
        cipher:String,

        #[structopt(short,long, help="The message to encode")]
        message:String,

        // optional so we can ignore it
        #[structopt(short,long, help="The shift value for caesar shift(default=3")]
        shift:Option<u8>,
    },

    #[structopt(name="decode",about="Decode a message using a cipher")]
    Decode {
        #[structopt(short,long, help="The cipher to use (e.g. rot13,reverse,caesar")]
        cipher:String,

        #[structopt(short,long, help="The message to encode")]
        message:String,

        // optional so we can ignore it
        #[structopt(short,long, help="The shift value for caesar shift(default=3")]
        shift:Option<u8>,
    }
}



fn main(){
    let opt = Cli::from_args();
    let result = match opt.cmd {
        Command::Encode { cipher, message, shift } =>  {
            match cipher.as_str() {
                "rot13" => rot13_encrypt(&message),
                "caesar" => {
                    let shift = shift.unwrap_or(3);
                    caesar_encrypt(&message,shift)
                },
                "reverse" => reverse_encrypt(&message),
                _ => "Unsupported cipher".to_string(),
            }
        }
        Command::Decode { cipher, message, shift } =>  {
            match cipher.as_str() {
                "rot13" => rot13_encrypt(&message),
                "caesar" => {
                    let shift = shift.unwrap_or(3);
                    caesar_encrypt(&message,shift) // decrypt
                },
                "reverse" => reverse_encrypt(&message),
                _ => "Unsupported cipher".to_string(),
            }
        }
    };
    // println!("{}",result);
    let mut output: Box<dyn Write> = if let Some(file_path) = &opt.output_file {
                Box::new(File::create(file_path).expect("Could not create file"))
            } else {
                // use a Box<dyn Write> to store either a File or Stdout. This allows us to treat both types uniformly because they both implement the Write trait.
                Box::new(stdout()) 
            };
        
    writeln!(output, "{:?}", result).expect("Could not write to output");
        
}


fn rot13_encrypt(text:&str) -> String{
    let encrypted_text = text.to_uppercase();
    encrypted_text
    .chars()
    .map(|c| match c {
        'A'..='M' => ((c as u8) + 13) as char,
        'N'..='Z' => ((c as u8) - 13) as char,
        _ => c,

    }).collect()
}

// Caesar Shift (rotX, rot16)
fn caesar_encrypt(text: &str, shift: u8) -> String{
    let mut encrypted_text = String::new();
    for char in text.chars(){
        if char.is_ascii_alphabetic(){
            let base = if char.is_ascii_uppercase() {'A'} else {'a'};
            let shifted = ((char as u8 - base as u8 + shift ) % 26) + base as u8;
            encrypted_text.push(shifted as char);
        } else {
            encrypted_text.push(char)
        }
    }
    encrypted_text

}

fn reverse_encrypt(text: &str)  ->String {
    text.chars().rev().collect()
}