
pub mod messages {
    use std::mem::size_of;
    #[derive(Debug, Copy)]
    #[repr(C)]

    pub struct Digital {
        pub mtype : u8,
        pub id : u16,
        pub pin : u16,
        pub rw : u8,
        pub action : u8
    }

    #[derive(Debug, Copy)]
    #[repr(C)]
    pub struct Analog {
        pub mtype : u8,
        pub id : u16,
        pub pin : u16,
        pub rw : u8,
        pub action : u16
    }

    #[derive(Debug, Copy)]
    #[repr(C)]
    pub struct Servo {
        pub mtype : u8,
        pub id : u16,
        pub pin : u16,
        pub rw : u8,
        pub degree : u8
    }


    impl Clone for Digital {
        fn clone(&self) -> Digital {*self}
    }

    impl Clone for Analog {
        fn clone(&self) -> Analog {*self}
    }

    impl Clone for Servo {
        fn clone(&self) -> Servo {*self}
    }


    #[repr(C)]
    pub union s_Analog {
        pub msg : Analog,
        pub serialized : [u8 ; size_of::<Analog>()]
    }

    #[repr(C)]
    pub union s_Digital {
        pub msg : Digital,
        pub serialized : [u8 ; size_of::<Digital>()]
    }

    #[repr(C)]
    pub union s_Servo {
        pub msg : Servo,
        pub serialized : [u8 ; size_of::<Servo>()]
    }
}