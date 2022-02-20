mod client;
mod error;
pub mod responses;

pub use crate::client::Client;

#[cfg(test)]
mod tests {
    use crate::Client;
    #[test]
    fn it_works() {
        let host = "http://localhost:3000".to_string();
        let access_key = "SCfPX4ngh6I70LgxPB3NUle6".to_string();
        let secret_key = "lY09vloC0jFzJMdyON7XXnYjRquscT+IZMbwY40fkPODR5aujJZQK+R4kx6R2R4kxGRXlhxGnjd+RBriaDQZpw==".to_string();
        let client = Client::new(host, access_key, secret_key).unwrap();
        let check = client.check_connection();
        println!("{check:?}");
        let operations = client.get_operations();
        println!("{operations:?}");
        let operation = client.get_operation(operations[0].id());
        println!("{operation:?}");
        let tags = client.tags_for_operation("HPSS");
        println!("{tags:?}");
    }
}
