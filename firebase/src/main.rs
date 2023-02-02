use dotenv::dotenv;
use fireauth::FireAuth;
// use fireauth::api::{SignUpResponse};

async fn mail_signup(auth: FireAuth, email: String, password: String) {
    let return_secure_token = true;
    println!(
        "Signup: {} with password {}, secure_token: {}",
        &email, &password, &return_secure_token
    );
    match auth.sign_up_email(&email, &password, return_secure_token).await {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(error) => {
            println!("Error:\n{error}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let api_key = std::env::var("API_KEY").expect("No API_KEY in .env file!");

    let auth = fireauth::FireAuth::new(api_key.clone());

    println!("Api: {}", &api_key);
    let user = std::env::var("TEST_USER").expect("No TEST_USER in .env file!");
    let psw  = std::env::var("PASSW").expect("No PASSW in .env file!");

    mail_signup(auth, user, psw).await;
}
