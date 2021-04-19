#![allow(non_snake_case)]

use uuid::Uuid;
fn main() {
    // let uuid = Uuid::nil();
    for i in 0..10 {
        let my_uuid = Uuid::new_v4();
        println!(
            "[{}] {} [{}] {} \n",
            i,
            my_uuid.simple(),
            my_uuid.simple(),
            my_uuid.hyphenated()
        );
    }
}
