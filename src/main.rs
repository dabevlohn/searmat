use dotenv::dotenv;
use matrix_sdk::{
    ruma::{
        api::client::keys::get_keys::v3::Request, api::client::keys::get_keys::v3::Response,
        api::client::user_directory::search_users::v3::User, UserId,
    },
    Client, HttpResult,
};
use std::{collections::BTreeMap, env};

struct UserProfiles {
    uids: Vec<User>,
    client: Client,
}

trait GetKeys {
    async fn run(&self, uid: &User) -> HttpResult<Response> {
        todo!();
    }
}

impl GetKeys for Client {
    async fn run(&self, uid: &User) -> HttpResult<Response> {
        let mut arg = BTreeMap::new();
        arg.insert(uid.user_id.to_owned(), vec![]);
        // dbg!(arg.clone());
        let mut request = Request::new();
        request.device_keys = arg;
        dbg!(request.clone());
        self.send(request, None).await
    }
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
    async fn get_keys(&self) {
        for u in self.uids.iter() {
            match self.client.run(u).await {
                Ok(r) => println!("{:?}", r),
                Err(e) => println!("{}", e),
            }
        }
    }
    // async fn print(&mut self) {}
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
    profiles.search("chope").await.get_keys().await;
    Ok(())
}
