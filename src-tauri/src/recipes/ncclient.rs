use reqwest_dav::{Auth, Client, ClientBuilder, Depth, list_cmd::ListEntity};
use async_trait::async_trait;

use super::{RecipesProvider, error::RecipesError};

pub struct NCClient {
    dav_client: Client

}

impl NCClient {
    pub fn new(host: String, username: String, password: String) -> Self {
        NCClient {
            dav_client: ClientBuilder::new()
                .set_host(format!("https://{}/remote.php/dav/files/{}/", host, username))
                .set_auth(Auth::Basic(username.to_owned(), password.to_owned()))
                .build().unwrap()
        }
    }
}

impl From<reqwest_dav::Error> for RecipesError {
    fn from(value: reqwest_dav::Error) -> Self {
        Self {reason: format!("{:?}", value)}
    }
}

#[async_trait]
impl RecipesProvider for NCClient {

    async fn list_recipes(&self) -> Result<Vec<String>, RecipesError> {
        let result : Vec<String> = self.dav_client.list(".TwiliRecipes", Depth::Number(1)).await?.into_iter()
            .filter(|le| match le { 
                ListEntity::File(_) => true,
                ListEntity::Folder(_) => false})
            .map(|le| 
                 if let ListEntity::File(file) = le {
                     file.href 
                 } else {
                     "".to_owned()
                 }
             ).collect();

        Ok(result)
    }
}
