use super::Group;

use leptos::prelude::*;
use leptos_use::use_media_query;
use thaw::*;

#[component]
pub fn GroupForm(group: Group, #[prop(into)] on_delete: Callback<()>) -> impl IntoView {
    let is_large_screen = use_media_query("(min-width: 600px)");
    let is_medium_screen = use_media_query("(min-width: 400px)");

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

    let input_len = Signal::derive(move || {
        is_medium_screen.with(|medium| if *medium { None } else { Some(6) })
    });
    view! {
        <Card>
            <CardHeader>
                <Field label="Group name"><Input value=group.name/></Field>
                <CardHeaderAction slot>
                    <Button appearance=ButtonAppearance::Secondary icon=icondata_bi::BiTrashRegular on:click=move |_| on_delete.run(())/>
                </CardHeaderAction>
            </CardHeader>
            <CardPreview class="p-2">
                <Text class="mb-6 text-lg">Ingredients</Text>
                <table>
                    <tr>
                        <td>Name</td><td>Qty.</td><td>Scale</td>
                    </tr>
                    <tr>
                        <td class="w-[60%]">
                            <Input size=input_size input_size=8 class="w-full" value=group.name/>
                        </td>
                        <td class="w-[5%]">
                            <SpinButton<f64> class="w-16" size=spin_size input_size=4 value=0.0 step_page=1.0/>
                        </td>
                        <td class="w-[30%]">
                           <Input size=input_size input_size=8 class="w-full" value=group.name/>
                        </td>
                        <td class="w-[5%]">
                            <Button appearance=ButtonAppearance::Transparent size=ButtonSize::Small icon=icondata_bi::BiTrashRegular />
                        </td>
                    </tr>
                </table>
            </CardPreview>
        </Card>
    }
}
