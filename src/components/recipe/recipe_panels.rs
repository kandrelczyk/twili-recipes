use leptos::prelude::*;
use recipes_common::Recipe;
use thaw::{
    Button, ButtonAppearance, ButtonShape, ButtonSize, Icon, Slider, Text, Toast, ToastBody,
    ToastOptions, ToastPosition, ToastStatus, ToastTitle, ToasterInjection,
};

use crate::components::{RecipeIngredients, RecipeStep};

fn format_time(time: i32) -> String {
    format!("{:02}:{:02}", time / 60, time % 60)
}

fn timer_tick(timer_secs: RwSignal<i32>, showing_toast: RwSignal<bool>) {
    timer_secs.update(|t| *t -= 1);
    if showing_toast.get() {
        set_timeout(
            move || timer_tick(timer_secs, showing_toast),
            std::time::Duration::from_secs(1),
        );
    }
}

#[component]
pub fn RecipePanels(recipe: Recipe) -> impl IntoView {
    let page = RwSignal::new(0.0);
    let page_count = recipe.steps.len() as f64 + 1.0;

    let multiplier = RwSignal::new(1.0);

    let recipe = StoredValue::new(recipe);
    let first_page = Signal::derive(move || page.get() == 0.0);
    let last_page = Signal::derive(move || page.get() as usize == recipe.get_value().steps.len());

    let toaster = ToasterInjection::expect_context();
    let toast_id = uuid::Uuid::new_v4();
    let showing_toats = RwSignal::new(false);

    let time = Memo::new(move |_| {
        if page.get() == 0.0 {
            0.0
        } else {
            recipe.get_value().steps[page.get() as usize - 1].time
        }
    });

    let timer_secs = RwSignal::new(time.get_untracked() as i32 * 60);

    let on_status_change = move |status| {
        showing_toats.set(status == ToastStatus::Mounted);
    };

    let start_timer = move |_| {
        timer_secs.set(time.get() as i32 * 60);
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>"Timer"</ToastTitle>
                        <ToastBody>
                            <Text class="text-xl flex flex-col items-center">
                                {move || format_time(timer_secs.get())}
                            </Text>
                        </ToastBody>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_position(ToastPosition::Bottom)
                .with_timeout(std::time::Duration::from_secs(timer_secs.get() as u64))
                .with_id(toast_id)
                .with_on_status_change(on_status_change),
        );
        set_timeout(
            move || timer_tick(timer_secs, showing_toats),
            std::time::Duration::from_secs(1),
        );
    };

    let stop_timer = move |_| {
        toaster.dismiss_toast(toast_id);
    };

    let step = Memo::new(move |_| {
        let mut step = recipe.get_value().steps[page.get() as usize - 1]
            .desc
            .clone();
        recipe.get_value().ingredients.iter().for_each(|i| {
            step = step.replace(
                format!("[{}]", i.name).as_str(),
                format!(
                    "{} ({}{})",
                    i.name,
                    i.quantity * multiplier(),
                    match i.scale.is_empty() {
                        true => "".to_owned(),
                        false => format!(" {}", i.scale),
                    }
                )
                .as_str(),
            )
        });
        step
    });
    view! {
        <div class="flex flex-col h-[90%] w-full">
            <div class="w-full flex flex-col overflow-auto items-center">
                {move || {
                    if page.get() == 0.0 {
                        view! { <RecipeIngredients recipe multiplier=multiplier /> }.into_any()
                    } else {
                        view! {
                            <div class="w-full text-xl text-center p-4">
                                Step {page}/ {page_count - 1.0}
                            </div>
                            <RecipeStep step=step.get() />
                            <Show when=move || {
                                time.get() > 0.0 || showing_toats.get()
                            }>
                                {move || {
                                    if !showing_toats.get() {
                                        view! {
                                            <Button
                                                on_click=start_timer
                                                shape=ButtonShape::Circular
                                                icon=icondata_bi::BiAlarmSolid
                                                appearance=ButtonAppearance::Primary
                                                size=ButtonSize::Large
                                                class="fixed bottom-8 right-8 z-[3000]"
                                            />
                                        }
                                    } else {
                                        view! {
                                            <Button
                                                on_click=stop_timer
                                                shape=ButtonShape::Circular
                                                icon=icondata_bi::BiAlarmOffSolid
                                                appearance=ButtonAppearance::Primary
                                                size=ButtonSize::Large
                                                class="fixed bottom-8 right-8 z-[3000]"
                                            />
                                        }
                                    }
                                }}
                            </Show>
                        }
                            .into_any()
                    }
                }}

            </div>
            <div class="grow"></div>
            <div class="flex max-w-4xl m-auto flex-row m-4 justify-center items-center">
                <div class="grow"></div>
                <Button
                    icon=icondata_bi::BiChevronLeftSolid
                    class="m-2"
                    on_click=move |_| page.update(|p| *p -= 1.0)
                    disabled=first_page
                ></Button>
                <Slider step=1.0 max=page_count - 1.0 value=page class="w-full m-4" />
                <Button
                    icon=icondata_bi::BiChevronRightSolid
                    class="m-2"
                    on_click=move |_| page.update(|p| *p += 1.0)
                    disabled=last_page
                ></Button>
                <div class="grow"></div>
            </div>
            {move || {
                if !last_page.get() {
                    view! {
                        <div
                            style="opacity: 0.35"
                            on:click=move |_| page.update(|p| *p += 1.0)
                            class="fixed h-full w-[15vw] bg-transparent top-0 right-0 sm:hidden block flex-col justify-center items-center"
                        >
                            <Icon
                                class="w-full h-full"
                                width="2em"
                                icon=icondata_bi::BiChevronRightSolid
                            />
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
            {move || {
                if !first_page.get() {
                    view! {
                        <div
                            style="opacity: 0.35"
                            on:click=move |_| page.update(|p| *p -= 1.0)
                            class="fixed h-full w-[15vw] top-0 left-0 bg-transparent sm:hidden block flex-col justify-center items-center"
                        >
                            <Icon
                                class="w-full h-full"
                                width="2em"
                                icon=icondata_bi::BiChevronLeftSolid
                            />
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
