

use rust_ipfs;

use rust_ipfs::ipns::*;

fn main() {

    // we need to open a port to listen to port 80.
    // we will get a http binary stream from the port.
    // it will contain IPFS Data.
    
    // the uploaded data will contain a content and a filename.
    
    let key_pair = rust_ipfs::Keypair::generate_secp256k1();


    
    key_pair.public_key();

    println!("PK: ", key_pair.public_key());

    let ipfs = Ipfs::new();

    
    rust_ipfs::ipns::Ipns::new(ipfs);
}
