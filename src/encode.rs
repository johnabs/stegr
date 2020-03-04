//use std::str;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use aes::Aes256;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn print_hello(_pass: String, _msg: String, _stegf: std::path::PathBuf, _imagepath: std::path::PathBuf, _rec: bool) {
    let encrypted = enc_msg(&_pass, _msg);
    println!("returned plaintext: {}", encrypted);
    let bo = store_msg(_pass, encrypted, _stegf, _imagepath);
//    println!("Hello, world!");
}

fn enc_msg(pass: &String, msg: String) -> String{
    //Hash password.
    let mut sha = Sha256::new();
    sha.input(pass.as_bytes());
    let output = sha.result();
    let key = output;
    let temp = output.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string();
  
    //Generate initialization vector from hash (maybe unsafe cryptographically, not sure).
    let mut iv: [u8;16] = Default::default();
    iv.copy_from_slice(&temp[..16].as_bytes());
    
    //Encrypt plaintext using hashed password and key
    let plaintext = msg.as_bytes();
    let cipher = Aes256Cbc::new_var(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);

    //Convert encrypted ciphertext to string formate and return.
    let temp = ciphertext.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string();
    temp
//    println!("Encrypted: {:?}", temp);
//    let cipher = Aes256Cbc::new_var(&key, &iv).unwrap();
//    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();
//    assert_eq!(decrypted_ciphertext, plaintext);
//    println!("Decrypted: {:?}", str::from_utf8(&decrypted_ciphertext).unwrap());
//
////    let seed: &[_] = &[1,2,3,4];
//    let mut rng: StdRng = SeedableRng::from_seed(seed);
}

fn store_msg(pass: String, encmsg: String, file: std::path::PathBuf, path: std::path::PathBuf) -> bool{
   // use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
   // let img = image::open(file).unwrap();
   // println!("dimensions {:?}", img.dimensions());
   // println!("{:?}", img.color());
    let mut flipper = format!("{:b}",encmsg.as_bytes()[0]).to_string();
    for i in 1..encmsg.as_bytes().len() {
        flipper = [flipper,format!("{:b}",encmsg.as_bytes()[i]).to_string()].join("");
    }
    println!("{}", encmsg);    
    println!("{}", flipper);    

    let mut sha = Sha256::new();
    sha.input(pass.as_bytes());
    let output = sha.result();
    let temp = output.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string();
    let mut seed: [u8;32] = Default::default();
    seed.copy_from_slice(&temp[..32].as_bytes());
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    println!("{}", rng.gen::<f64>());    
    let mut my_array = vec![0.0_f64; 100];
    for i in 0..my_array.len() {
        my_array[i] =rng.gen::<f64>();
    }
 //   println!("{:?}", my_array);

    
    true
 }

fn bin_str_to_str(input: String) -> String{
    let mut outstr = "";
    let mut temp = "";
    for i in 0..input.chars().count()-1 {
        temp = [temp,input[i]].join("");
        outstr = format!("{:u8}",) ;
    }
    


}
