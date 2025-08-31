use super::Group;

use crate::components::{IngredientForm, IngredientValue};
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn GroupForm(group: Group, #[prop(into)] on_delete: Callback<()>) -> impl IntoView {
    let on_delete_ing = move |i| {
        log::info!("delete {i}");
        group.ingredients.update(|ing| {
            ing.remove(i);
        });
    };

    let ingredients_form = move || {
        group
            .ingredients
            .get()
            .into_iter()
            .enumerate()
            .map(|(i, ing)| {
                view! { <IngredientForm ingredient=ing on_delete=move || on_delete_ing(i) /> }
                    .into_any()
            })
            .collect::<Vec<AnyView>>()
    };
    let add_ingredient = move |_| {
        group.ingredients.update(|ing| {
            ing.push(IngredientValue {
                name: RwSignal::new("".to_owned()),
                scale: RwSignal::new("".to_owned()),
                quantity: RwSignal::new(1.0),
            });
        });
    };

    view! {
        <Card>
            <CardHeader>
                <Field label="Group name">
                    <Input value=group.name />
                </Field>
                <CardHeaderAction slot>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        icon=icondata_bi::BiTrashRegular
                        on:click=move |_| on_delete.run(())
                    />
                </CardHeaderAction>
            </CardHeader>
            <CardPreview class="p-2">
                <Text class="mb-6 text-lg">Ingredients</Text>
                <table>
                    <tr>
                        <td>Name</td>
                        <td>Qty.</td>
                        <td>Scale</td>
                    </tr>
                    {ingredients_form}
                    <Button
                        class="mt-2"
                        on:click=add_ingredient
                        size=ButtonSize::Small
                        icon=icondata_bi::BiPlusRegular
                    >
                        Add ingredient
                    </Button>
                </table>
            </CardPreview>
        </Card>
    }
}
