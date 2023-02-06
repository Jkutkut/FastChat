use dotenv::dotenv as dotenvparser;
use fireauth::FireAuth;
// use fireauth::api::{SignUpResponse};

async fn mail_signup(auth: &FireAuth, email: &str, password: &str) {
    let return_secure_token = true;
    println!(
        "Signup: {} with password {}, secure_token: {}",
        email, password, &return_secure_token
    );
    match auth.sign_up_email(email, password, return_secure_token).await {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(error) => {
            println!("Error:\n{error}");
        }
    }
}

async fn mail_signin(auth: &FireAuth, email: &str, password: &str) {
    let return_secure_token = true;
    println!(
        "Signin: {} with password {}, secure_token: {}",
        email, password, &return_secure_token
    );
    match auth.sign_in_email(email, password, return_secure_token).await {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(error) => {
            println!("Error:\n{error}");
        }
    }
}

async fn user_info(auth: &FireAuth, id_token: &str) {
    println!(
        "User info: {}",
        id_token
    );
    match auth.get_user_info(id_token).await {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(error) => {
            println!("Error:\n{error}");
        }
    }
}

fn dotenv(key: &str) -> String {
    std::env::var(key)
        .expect(format!("Key: {key} not found on .env file").as_str())
}

#[tokio::main]
async fn main() {
    dotenvparser().ok(); // Load .env file

    let api_key = dotenv("API_KEY");
    let auth = fireauth::FireAuth::new(api_key.clone());

    let user = dotenv("EMAIL");
    let psw = dotenv("PASSW");

    println!("\n");
    mail_signup(&auth, &user, &psw).await;
    println!("\n");
    mail_signin(&auth, &user, &psw).await;
    println!("\n");
    //user_info(&auth, &dotenv("id_token")).await;
}
