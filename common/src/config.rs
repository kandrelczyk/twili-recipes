use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LLM {
    Copilot,
    GPT,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RecipesSource {
    Cloud,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub llm: LLM,
    pub ai_token: String,
    pub ai_prompt: String,
    pub recipes_source: RecipesSource,
    pub cloud_uri: String,
    pub cloud_username: String,
    pub cloud_pass: String,
}

impl Config {
    pub fn all_present(&self) -> bool {
        !(self.ai_token.is_empty()
            || self.cloud_uri.is_empty()
            || self.cloud_username.is_empty()
            || self.ai_prompt.is_empty())
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            llm: LLM::Copilot,
            ai_token: "".to_owned(),
            ai_prompt: r#"You are a cooking assisatnt. Users send you recipes and you transofrm them into valid JSON. For each recipe received from users you do the following: 
                1. keep in the original language 
                2. transform it into valid JSON of the following format: {"ingredients": [{"name": "", "quantity": 1.5, "scale": ""}], "steps": [desc: "", time: 1] } for example: {"ingredients": [{"name": "eggs", "quantity": 0.5, "scale": ""}, {"name": "milk", "quantity": 200, "scale": "ml"}], "steps": [{desc: "", time: 1}]} 
            3. when writing steps replace the ingredients with <name> where name is one of the ingredient listed at the beginning. If ingredient is not listed in the initial list keep it in it`s original form. Make sure the only names listed in the "ingredients" are used in the body. For example if the recipe says "Ingredients: 12 Chicken Eggs. Step 1. Add milk and eggs" change it to "Add milk and <Chicken Eggs>". "Milk" is not changed to "<Milk>" because it`s not listed as ingredient. 
                4. return only the JSON as response without any whitespace characters. 
                5. For quantities provided as fractions change then to decimal values. 
                6. If a step contains some execution or wait time (for example "bake for 15 minutes") "time" properoty of a given step should be equal to this time. Otherise it should be 0."#.to_owned(),
            recipes_source: RecipesSource::Cloud,
            cloud_uri: "".to_owned(),
            cloud_username: "".to_owned(),
            cloud_pass: "".to_owned(),
        }
    }
}
