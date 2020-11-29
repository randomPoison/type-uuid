use type_uuid::TypeUuid;
use uuid::Uuid;

#[derive(TypeUuid)]
#[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
struct MyType;

#[derive(TypeUuid)]
#[uuid = "484d7f8c-65a1-4c77-96d1-3e8447c5bf5e"]
struct MyGenericType<T: TypeUuid> {
    pub field: Vec<T>,
}

fn main() {
    println!("MyType: {}", Uuid::from_bytes(MyType::UUID));
    println!(
        "MyType: {}",
        Uuid::from_bytes(MyGenericType::<String>::UUID)
    );
}
