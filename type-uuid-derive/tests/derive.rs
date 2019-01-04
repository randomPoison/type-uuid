use type_uuid_derive::*;
use type_uuid::TypeUuid;

#[test]
fn derive() {
    #[derive(TypeUuid)]
    #[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
    struct MyType;

    println!("Uuid: {:?}", MyType::UUID);
}
