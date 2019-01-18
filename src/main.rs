


#[macro_use] 
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

extern crate spidev;

use std::thread;
use std::time::Duration;


extern crate i2cdev;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

    
pub mod ll_io {
    // add code here
    use spidev::{Spidev, SpidevOptions, SPI_MODE_0};
    use spidev::spidevioctl::SpidevTransfer;


    extern crate i2cdev;
    use i2cdev::core::*;
    use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

    const SLAVE_ADDR: u16 = 0x04;

    pub struct Spi {
        dev: Spidev,
        logging : bool
    }

    impl Spi {
            pub fn new(_logging : bool) -> Spi{
                let mut spidev = Spidev::open("/dev/spidev0.0").unwrap();
                let options = SpidevOptions::new()
                                  .bits_per_word(8)
                                  .max_speed_hz(5000)
                                  .lsb_first(false)
                                  .mode(SPI_MODE_0)
                                  .build();

                spidev.configure(&options).unwrap();
                Spi{ dev: spidev,
                    logging : _logging}
            }


        pub fn spi_write_respond(&self, _payload : &[u8]) -> String
        {
                // it will probobalby be worth it to some timing tests for wait on the transfer
            let mut rx_buf : Vec<u8> = Vec::with_capacity(_payload.len());

            for _i in 0.._payload.len() {
                rx_buf.push('\0' as u8);
            }

            let mut transfer = SpidevTransfer::read_write(&_payload, &mut rx_buf);
            
            let response = self.dev.transfer(&mut transfer);

            match response {
                Ok(_) => {match String::from_utf8(rx_buf){
                    Ok(n) => return n,
                    Err(e) => {if self.logging {error!("String failed to be created {:?} {} {}", e, file!(), line!())}
                        return String::new();}
                    }
                },
                Err(e) => {
                        if self.logging { error!("transfer Failed {:?} {} {}", e, file!(), line!())};
                        return String::new();
                },
            }

        }
    }
}

fn send_via_i2c(i2cdev : &mut LinuxI2CDevice, msg : &Vec<u8>){
    for byte in msg.iter() {
        match (*i2cdev).smbus_write_byte_data(0x04, *byte){
            Ok(_) => continue,
            Err(e)=> println!("{:?}", e)
        }

    thread::sleep(Duration::from_millis(10));
    }
}
fn main() {

        CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let _s_dev = ll_io::Spi::new(true);
    let tx_buf = "d 24 w 1 \n".as_bytes();

    println!("{:?}",_s_dev.spi_write_respond(tx_buf));

    loop {
    //     let mut rx_buf = [0_u8 ; 10];
        let _tx_buf = "d 24 w 1 \n".as_bytes();
        thread::sleep(Duration::from_millis(10));
        println!("{:?}",_s_dev.spi_write_respond("this is a bigger string i think".as_bytes()));

    }
}