use structopt::StructOpt;

// Structs
#[derive(Debug,StructOpt)]
#[structopt(name="cipherize", about="A Simple CLI for Cipher and Crypto")]
struct Opt {
    // This will be ignored as a help message but /// is a documentation
    /// Cipher a text
    #[structopt(subcommand)]
    cmd: Command,
}

// Enums
#[derive(Debug,StructOpt)]
enum Command {
    Encode{
        #[structopt(short="c", long,default_value="rot13")]
        cipher:String  
    }
}


fn main() {
   let opt = Opt::from_args();
   println!("{:?}",opt);
}
