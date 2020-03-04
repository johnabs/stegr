use structopt::StructOpt;
mod encode;
mod decode;
#[macro_use] extern crate hex_literal;
extern crate block_modes;

#[derive(StructOpt)]
#[structopt(name="stegr")]
struct Cli {
    /// The type of cryptography to perform, encoding or decoding are the options.
    #[structopt(long = "crypt", short = "c")]
    crypt: String,
    /// The password you want to use. 
    #[structopt(long = "password", short = "p")]
    password: String,
    /// The message to encode if using a string message
    #[structopt(long="message", short="m")]
    message: String,
    /// The file to encode if using a file message
    #[structopt(long="stegfile", short="s",parse(from_os_str))]
    stegfile: std::path::PathBuf,
    /// The path to the image(s) to hide your message in 
    #[structopt(long = "imagepath", short = "i",parse(from_os_str))]
    imagepath: std::path::PathBuf,
    /// Flag whether or not to recursively hide your message across multiple images in 1
    /// folder or multiple images across multiple folders (depends on file struct).
    #[structopt(long="recursive", short="r")]
    recursive: bool 
}

fn main() {
	let args = Cli::from_args();

    if args.crypt.contains("enc"){
        encode::print_hello(args.password, args.message, args.stegfile, args.imagepath, args.recursive);
    }
    else if args.crypt.contains("dec"){ 
        decode::print_goodbye(args.password, args.message, args.stegfile, args.imagepath, args.recursive);
    }
}
