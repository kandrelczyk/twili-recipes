use leptos::prelude::*;
use leptos_use::use_media_query;
use thaw::*;

use crate::components::IngredientValue;

#[component]
pub fn IngredientForm(
    ingredient: IngredientValue,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let is_large_screen = use_media_query("(min-width: 600px)");

    let input_size = Signal::derive(move || {
        is_large_screen.with(|large| {
            if *large {
                InputSize::Medium
            } else {
                InputSize::Small
            }
        })
    });
    let spin_size = Signal::derive(move || {
        is_large_screen.with(|large| {
            if *large {
                SpinButtonSize::Medium
            } else {
                SpinButtonSize::Small
            }
        })
    });
    view! {
        <tr class="align-top">
            <td class="w-[60%]">
                <Field required=true>
                    <Input
                        rules=vec![
                            InputRule::required_with_message(
                                true.into(),
                                "Provide name".to_owned().into(),
                            ),
                        ]
                        size=input_size
                        input_size=8
                        class="w-full"
                        value=ingredient.name
                    />
                </Field>
            </td>
            <td class="w-[5%]">
                <SpinButton<
                f32,
            > class="w-16" size=spin_size input_size=4 value=ingredient.quantity step_page=1.0 />
            </td>
            <td class="w-[30%]">
                <Input size=input_size input_size=8 class="w-full" value=ingredient.scale />
            </td>
            <td class="w-[5%]">
                <Button
                    on:click=move |_| on_delete.run(())
                    appearance=ButtonAppearance::Transparent
                    size=ButtonSize::Small
                    icon=icondata_bi::BiTrashRegular
                />
            </td>
        </tr>
    }
}
