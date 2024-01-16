use base64::{encode, decode};

//encode() and decode() are deprecated but the reason is not of great essence here in my opinion.
//We only use base64 to avoid SQL-injections so default mode serves us well enough.
//Check https://github.com/marshallpierce/rust-base64/issues/213 for further information.
    
pub fn encode_base64<T>(input: T) -> String where T: AsRef<[u8]> {
    encode(input)
}

pub fn decode_base64<T>(input: T) -> String where T: AsRef<[u8]>{
    String::from_utf8(decode(input).unwrap()).expect("Item could not be processed.")
}