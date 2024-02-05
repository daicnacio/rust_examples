use prost::Message;
// Include the `items` module, which is generated from items.proto.
// ts is the name of the package in items.proto
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/ts.rs"));
}
fn create_ts()->items::TimeSeries{
    let mut ts = items::TimeSeries::default();
    ts.measurement = vec![items::Measurement{wind_x:1.1,wind_y:2.3,timestamp:1},items::Measurement{wind_x:1.5,wind_y:2.5,timestamp:2}];
    ts
    
}

fn serialize_ts( meas : &items::TimeSeries)->Vec<u8>{
    let mut buf = Vec::new();
    buf.reserve(meas.encoded_len());
    meas.encode(&mut buf).unwrap();
    buf
}
pub fn deserialize_ts(mut buf: &[u8]) -> Result<items::TimeSeries, prost::DecodeError> {
    let s = items::TimeSeries::decode(&mut buf);
    s
}

fn main() {
    let my_meas = create_ts();
    let request_vector = serialize_ts(&my_meas);
    let output_val = deserialize_ts(&request_vector).unwrap();
    println!("{:?}",output_val);

    println!("Hello, world!");
}
