use actix_web::client::Client;

#[actix_rt::main]
async fn main() {
   let client = Client::default();

   // Create request builder and send request
   let response = client.get("http://www.rust-lang.org")
      .header("User-Agent", "Actix-web")
      .send().await;                      // <- Send http request

   println!("Response: {:?}", response);
}

// #[test]
// fn should_fail() {
//     unimplemented!();
// }