use super::Step;

use leptos::prelude::*;
use thaw::*;

#[component]
pub fn StepForm(step: Step, index: usize, #[prop(into)] on_delete: Callback<()>) -> impl IntoView {
    view! {
        <Card>
            <CardPreview class="p-2">
                <div class="flex flex-row w-full gap-2">
                    <Field class="w-full" label=format!("Step {}", index + 1)>
                        <Textarea
                            class="w-full min-h-[80px]"
                            size=TextareaSize::Large
                            rules=vec![TextareaRule::required(true.into())]
                            value=step.desc
                        />
                    </Field>
                    <div>
                        <Button
                            size=ButtonSize::Medium
                            appearance=ButtonAppearance::Secondary
                            icon=icondata_bi::BiTrashRegular
                            on:click=move |_| on_delete.run(())
                        />
                    </div>
                </div>
                <Field label="Time" class="mt-2">
                    <SpinButton<u32> class="w-16" input_size=4 value=step.time step_page=5 />
                </Field>
            </CardPreview>
        </Card>
    }
}
