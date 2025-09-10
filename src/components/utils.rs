use std::collections::BTreeMap;

use recipes_common::Recipe;

pub fn group_ingredients(recipe: &Recipe) -> BTreeMap<String, Vec<recipes_common::Ingredient>> {
    let mut groups = BTreeMap::<String, Vec<recipes_common::Ingredient>>::new();
    for ingredient in recipe.ingredients.iter() {
        let group = groups.entry(ingredient.group.clone().unwrap_or_default());
        group
            .and_modify(|g| g.push(ingredient.clone()))
            .or_insert(vec![ingredient.clone()]);
    }

    groups
}
