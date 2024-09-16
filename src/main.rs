mod database;

use std::thread::sleep;
use std::time::Duration;

use nfc1::*;

const MAX_FRAME_LEN: usize = 264;
const UNLOCK_1: [u8; 1] = [0x40];
const UNLOCK_2: [u8; 1] = [0x40];

fn main() {

    println!("helo woww");
    println!("nfc version v{}", nfc1::version());

    let mut context = Context::new().expect("context failed");
    let mut device = context.open().expect("opening connection failed");

    println!("nfc reader: {} opened", device.name());
    device.initiator_init().expect("error while initiating");

    loop {
        println!("looking for targets...");

        match device.initiator_select_passive_target(&Modulation{
            modulation_type: ModulationType::Iso14443a,
            baud_rate: BaudRate::Baud106,
        }) {
            Ok(target) => {
                println!("target found: {}", target.to_string(false).expect("error while getting target name"));
            
                match device.initiator_transceive_bits(&UNLOCK_1, 7, MAX_FRAME_LEN) {
                    Ok(rx) => {
                        println!("received bits: {:02X?}", rx);
                    },
                    Err(err) => {
                        println!("this card is not compatible. {:?}", err);
                        sleep(Duration::from_secs(5));
                        println!("looking for targets");
                        continue;
                    },
                };

                match device.initiator_transceive_bytes(&UNLOCK_2, MAX_FRAME_LEN, Timeout::Default) {
                    Ok(rx) => {
                        println!("received bytes: {:02X?}", rx);
                        println!("this card is compatible");
                    },
                    Err(err) => {
                        println!("this card is not compatible. {:?}", err);
                    }
                }
            },
            Err(_) => { continue; }
        }
        sleep(Duration::from_secs(5));
    }

}
