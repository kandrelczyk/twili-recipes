use async_trait::async_trait;
use recipes_common::{ListEntry, Recipe};
use reqwest_dav::{Auth, Client, ClientBuilder};

use super::{error::RecipesError, RecipesProvider};

pub struct NCClient {
    dav_client: Client,
    path: String,
}

impl NCClient {
    pub fn new(host: String, username: String, password: String) -> Self {
        let hostname = match !host.starts_with("https://") && !host.starts_with("http://") {
            true => format!("https://{}", host),
            false => host,
        };

        NCClient {
            dav_client: ClientBuilder::new()
                .set_host(format!("{}/remote.php/dav/files/{}/", hostname, username))
                .set_auth(Auth::Basic(username.to_owned(), password.to_owned()))
                .build()
                .unwrap(),
            path: ".TwiliRecipes".to_owned(),
        }
    }
}

impl From<reqwest_dav::Error> for RecipesError {
    fn from(value: reqwest_dav::Error) -> Self {
        Self {
            reason: format!("{:?}", value),
        }
    }
}

impl From<reqwest_dav::re_exports::reqwest::Error> for RecipesError {
    fn from(value: reqwest_dav::re_exports::reqwest::Error) -> Self {
        Self {
            reason: format!("{:?}", value),
        }
    }
}

static LIST_FILE_NAME: &str = ".list.json";

impl NCClient {
    async fn save_list(&self, list: &Vec<ListEntry>) -> Result<(), RecipesError> {
        let recipe_json: String = serde_json::to_string(list)?;
        self.dav_client
            .put(&format!("{}/{}", self.path, LIST_FILE_NAME), recipe_json)
            .await?;

        Ok(())
    }

    async fn add_to_list(&self, recipe: &Recipe) -> Result<(), RecipesError> {
        let mut recipes: Vec<ListEntry> = self.list_recipes().await?;

        let id = recipe.id.as_ref().unwrap().clone();
        let recipe_entry = recipes.iter_mut().find(|r| r.filename.eq(&id));

        match recipe_entry {
            Some(entry) => entry.name.clone_from(recipe.name.as_ref().unwrap()),
            None => recipes.push(ListEntry {
                name: recipe.name.as_ref().expect("Recipe name not set").clone(),
                filename: id,
            }),
        }

        self.save_list(&recipes).await?;

        Ok(())
    }

    async fn delete_from_list(&self, filename: &String) -> Result<(), RecipesError> {
        let mut recipes: Vec<ListEntry> = self.list_recipes().await?;

        recipes.retain(|f| f.filename != *filename);

        self.save_list(&recipes).await?;

        Ok(())
    }
}

#[async_trait]
impl RecipesProvider for NCClient {
    async fn list_recipes(&self) -> Result<Vec<ListEntry>, RecipesError> {
        let response = self
            .dav_client
            .get(&format!("{}/{}", self.path, LIST_FILE_NAME))
            .await;

        if response.is_err() && format!("{:?}", response).contains("response_code: 404") {
            self.save_list(&Vec::<ListEntry>::new()).await?;
            return Ok(Vec::new());
        }

        let recipes: Vec<ListEntry> = serde_json::from_str(&response?.text().await?)?;
        Ok(recipes)
    }

    async fn save_recipe(&mut self, mut recipe: Recipe) -> Result<(), RecipesError> {
        if recipe.id.is_none() {
            recipe.id = Some(uuid::Uuid::new_v4().to_string());
        }

        let recipe_json: String = serde_json::to_string(&recipe)?;
        self.dav_client
            .put(
                &format!(
                    "{}/{}",
                    self.path,
                    recipe.id.as_ref().expect("Missing filename")
                ),
                recipe_json,
            )
            .await?;

        self.add_to_list(&recipe).await?;

        Ok(())
    }
    async fn delete_recipe(&mut self, filename: String) -> Result<(), RecipesError> {
        let response = self
            .dav_client
            .delete(&format!("{}/{}", self.path, filename))
            .await;

        if response.is_err() && format!("{:?}", response).contains("response_code: 404") {
            return Err(RecipesError {
                reason: format!("Failed to delete recipe: {:?}", response),
            });
        }
        self.delete_from_list(&filename).await?;

        Ok(())
    }

    async fn get_recipe(&self, filename: String) -> Result<Recipe, RecipesError> {
        let response = self
            .dav_client
            .get(&format!("{}/{}", self.path, filename))
            .await;

        if response.is_err() && format!("{:?}", response).contains("response_code: 404") {
            return Err(RecipesError {
                reason: format!("File {} not found", filename),
            });
        }

        let recipe: Recipe = serde_json::from_str(&response?.text().await?)?;
        Ok(recipe)
    }
}
