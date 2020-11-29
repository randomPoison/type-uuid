use type_uuid::TypeUuid;
use uuid::Uuid;

pub fn main() {
    println!("Vec<u32>: {}", Uuid::from_bytes(Vec::<u32>::UUID));
    // println!("Box<str>: {}", Uuid::from_bytes(Box::<str>::UUID));
}
