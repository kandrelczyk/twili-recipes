use std::collections::HashMap;

use recipes_common::Recipe;

pub fn group_ingredients(recipe: &Recipe) -> HashMap<String, Vec<recipes_common::Ingredient>> {
    let mut groups = HashMap::<String, Vec<recipes_common::Ingredient>>::new();
    for ingredient in recipe.ingredients.iter() {
        let group = groups.entry(ingredient.group.clone().unwrap_or_default());
        group
            .and_modify(|g| g.push(ingredient.clone()))
            .or_insert(vec![ingredient.clone()]);
    }

    groups
}
