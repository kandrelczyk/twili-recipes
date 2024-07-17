use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LLM {
    Copilot,
    GPT,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RecipesSource {
    Cloud,
    Local,
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
        match self.recipes_source {
            RecipesSource::Cloud => {
                !(self.ai_token.is_empty()
                    || self.cloud_uri.is_empty()
                    || self.cloud_username.is_empty()
                    || self.ai_prompt.is_empty())
            }
            RecipesSource::Local => !self.ai_token.is_empty(),
        }
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            llm: LLM::GPT,
            ai_token: "".to_owned(),
            ai_prompt: r#"
            You are a cooking assistant. Users send you recipes and you transform them into valid JSON. For each recipe received from users you do the following:
  1. Keep it in the original language                                             
  2. Transform it into valid JSON of the following format: {"ingredients": [{"name": "", "quantity": 1.5, "scale": "", "group": ""}], "steps": [] } for example: {"ingredients": [{"name": "eggs", "quantity": 0.5, "scale": "", "group": "dough"}, {"name": "milk", "quantity": 200, "scale": "ml", "group": "dough"}], "steps": [{"desc": "", "time": 1}]}
  3. If ingredients are split into groups include "group" property with corresponding value. Otherwise leave empty.
  4. In description of steps surround name of all ingredients with '[]'. Make sure that name in the list of ingredients and in the steps match exactly. Change grammatical form if necessary. 
  5. Return only the JSON as response. Add standard indentation. Never include 'json' prefix at the begging.
  6. For quantities provided as fractions change them to decimal values.          
  7. If a step contains some execution or wait time (for example "bake for 15 minutes") "time" property of a given step should be equal to this time. Otherwise it should be 0.
"#.to_owned(),
            recipes_source: RecipesSource::Cloud,
            cloud_uri: "".to_owned(),
            cloud_username: "".to_owned(),
            cloud_pass: "".to_owned(),
        }
    }
}
