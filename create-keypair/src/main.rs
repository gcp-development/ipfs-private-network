use libp2p::{
    core::identity::Keypair,
    PeerId
};

use std::{
    io::{Read, Write},
    fs::File
};

fn main() {
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let encoding = local_key.to_protobuf_encoding();
    match encoding {
        Ok(write_buffer) => {
            let keypair_file = File::create("keypair.bin");
            match keypair_file {
                Ok(mut file) => {
                    let _ = file.write_all(&write_buffer);
                }
                Err(_) => {
                    println!("File not created...")
                }
            }
            let peer_id_file = File::create("peer_id.bin");
            match peer_id_file {
                Ok(mut file) => {
                    let _ = file.write_all(&local_peer_id.to_bytes());
                }
                Err(_) => {
                    println!("File not created...");
                }
            }
        }
        Err(_) => {
            println!("Encoding failed...");
        }
    }

    let mut keypair_file_buffer = Vec::new();
    let keypair_file = File::open("keypair.bin");
    match keypair_file {
        Ok(mut file) => {
            file.read_to_end(&mut keypair_file_buffer).expect("keypair.bin");
        }
        Err(_) => {
            println!("Error reading file keypair.bin.");
        }
    }

    let read_keypair = Keypair::from_protobuf_encoding(&keypair_file_buffer);
    println!("Keypair: {:?}", read_keypair);

    let mut peer_id_file_buffer = Vec::new();
    let read_file = File::open("peer_id.bin");
    match read_file {
        Ok(mut file) => {
            file.read_to_end(&mut peer_id_file_buffer).expect("peer_id.bin");
        }
        Err(_) => {
            println!("Error reading file peer_id.bin.");
        }
    }

    let read_peer_id= PeerId::from_bytes(&peer_id_file_buffer);
    println!("PeerId: {:?}", read_peer_id);
}