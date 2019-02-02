
#![feature(untagged_unions)]

use testcomm::messages::messages::*;
use testcomm::ll_io::*;
use std::mem::size_of;





fn main() {



    let _d = Digital{mtype : 'd' as u8, id : 123, pin : 23, rw : 'w' as u8, action : 1  };
    let _sd= s_Digital{msg : _d.clone()};

    let _a = Analog{mtype : 'a' as u8, id : 123, pin : 23, rw : 'w' as u8, action : 1234 };
    let _sa= s_Analog{msg : _a.clone()};

    let _s = Servo{mtype : 's' as u8, id : 123, pin : 23, rw : 'w' as u8, degree : 124 };
    let _ss= s_Servo{msg : _s.clone()};

    let _s_com = ll_io::Spi::new(false, '\n' as u8);
    let _c_com = ll_io::ItwoC::new(true, '\n' as u8, 0x8);
    println!("{:?}", unsafe{_ss.msg} );

    println!("{:?}", unsafe{_ss.serialized} );
    println!("{:?}", _s_com.write_respond(unsafe{ &_ss.serialized} ));
    println!("{:?}", _c_com.write_respond(unsafe{ &_ss.serialized} ));


}