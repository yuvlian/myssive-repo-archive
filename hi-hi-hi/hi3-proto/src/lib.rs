pub use prost::Message as Conv;

pub fn decode<T: Conv + Default>(data: &[u8]) -> T {
    T::decode(data).unwrap_or_else(|_| {
        println!("->> failed decoding to msg, defaulting");
        T::default()
    })
}

pub mod cmd {
    include!("../out/cmd.rs");
}

include!("../out/_.rs");
