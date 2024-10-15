use dotenv::dotenv;
use matrix_sdk::{
    ruma::{api::client::user_directory::search_users::v3::User, UserId},
    Client,
};
use std::env;

struct UserProfiles {
    uids: Vec<User>,
    client: Client,
}

impl UserProfiles {
    async fn init(u: &UserId, p: &str) -> Self {
        let me = User::new(u.to_owned());
        let client = Client::builder()
            .server_name(u.server_name())
            .build()
            .await
            .unwrap();
        client
            .matrix_auth()
            .login_username(u, p)
            .send()
            .await
            .unwrap();
        Self {
            uids: vec![me],
            client,
        }
    }
    async fn search(&mut self, term: &str) -> &mut Self {
        let response = self.client.search_users(term, 50).await;
        match response {
            Ok(r) => {
                for u in r.results {
                    self.uids.push(u);
                }
            }
            Err(e) => println!("{}", e),
        }
        self
    }
    // async fn get_keys(&self) {}
    async fn print(&mut self) {
        for u in self.uids.iter() {
            dbg!(u);
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let user = match env::var("MATRIXUSER") {
        Ok(v) => v,
        Err(_) => "Error loading env variable".to_owned(),
    };
    let pass = match env::var("MATRIXPASS") {
        Ok(v) => v,
        Err(_) => "Error loading env variable".to_owned(),
    };
    let u = <&UserId>::try_from(user.as_str()).unwrap();
    let mut profiles = UserProfiles::init(u, &pass).await;
    profiles.search("chope").await.print().await;
    Ok(())
}
