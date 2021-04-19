use uuid::Uuid;

pub fn api_guid(){
    let my_uuid = Uuid::new_v4();
    println!("{}\n",my_uuid.simple());
}