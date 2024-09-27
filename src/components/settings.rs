use leptos::prelude::*;
use leptos::spawn::spawn_local;

use leptos_router::hooks::use_navigate;
use recipes_common::{Config, RecipesSource};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use wasm_bindgen::prelude::*;

use crate::{
    components::{Header, LLMInfo},
    error::CommandError,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Args {
    config: Config,
}

#[component]
pub fn Settings(init: bool) -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());
    let loading = RwSignal::new(false);
    let toaster = ToasterInjection::expect_context();
    let command_error: RwSignal<Option<CommandError>> = RwSignal::new(None);

    let has_config = RwSignal::new(init);
    let llm_service = RwSignal::new("Free".to_owned());
    let llm_token = RwSignal::new("".to_owned());

    let cloud_storage = RwSignal::new(false);
    let cloud_uri = RwSignal::new("".to_owned());

    let cloud_username = RwSignal::new("".to_owned());

    let cloud_pass = RwSignal::new("".to_owned());

    let show_info = RwSignal::new(false);

    if !init {
        let listener = window_event_listener_untyped("popstate", move |_| {
            navigate.get_untracked()("/list", Default::default())
        });
        on_cleanup(|| listener.remove());
        spawn_local(async move {
            match invoke("get_config", JsValue::NULL).await {
                Ok(config) => {
                    let config: Config = from_value(config).unwrap();
                    llm_service.set(format!("{}", config.llm));
                    cloud_storage.set(matches![config.recipes_source, RecipesSource::Cloud]);
                    llm_token.set(config.ai_token);
                    cloud_uri.set(config.cloud_uri);
                    cloud_username.set(config.cloud_username);
                    cloud_pass.set(config.cloud_pass);
                    has_config.set(true);
                }
                Err(err) => command_error.set(Some(from_value::<CommandError>(err).unwrap())),
            };
        });
    }

    view! {
        <main class="flex flex-col h-screen w-full items-center justify-start">
            <Dialog open=show_info>
                <LLMInfo/>
            </Dialog>
            <Header
                button=move || {
                    if init {
                        view! { "" }.into_any()
                    } else {
                        view! {
                            <Button
                                class="ml-1 absolute"
                                appearance=ButtonAppearance::Subtle
                                shape=ButtonShape::Circular
                                icon=icondata_bi::BiChevronLeftSolid
                                on:click=move |_| {
                                    navigate.get_untracked()("/list", Default::default())
                                }
                            ></Button>
                        }
                            .into_any()
                    }
                }

                title=move || { if init { "Initial setup" } else { "Settings" } }
            />
            {move || {
                if init || has_config.get() {
                    view! {
                        <FieldContextProvider>
                            <div class="flex flex-col items-center h-full w-full">
                                <div class="p-2 w-full max-w-xl h-full">
                                    <div id="lmm_service" class="p-2 text-sm w-full">
                                        <div class="flex flex-row gap-1">
                                            <Text>"LLM Service"</Text>
                                            <Button
                                                on_click=move |_| show_info.set(true)
                                                appearance=ButtonAppearance::Transparent
                                                size=ButtonSize::Small
                                                icon=icondata_bi::BiInfoCircleRegular
                                            ></Button>
                                        </div>
                                        <RadioGroup value=llm_service class="p-2">
                                            <Radio value="Free" label="Free" />
                                            <Radio value="GPT" label="OpenAI" />
                                            <Radio value="Perplexity" label="Perplexity" />
                                        </RadioGroup>
                                        {move || match llm_service.get().as_str() {
                                            "Free" => {
                                                view! {
                                                    <div>
                                                        Use shared, rate limited LLM service. Click
                                                        <Icon icon=icondata_bi::BiInfoCircleRegular /> for more info.
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                            "OpenAI" => {
                                                view! {
                                                    <Field label="ChatGPT API Token" required=true>
                                                        <Input
                                                            class="w-full"
                                                            value=llm_token
                                                            disabled=loading
                                                            rules=vec![
                                                                InputRule::required_with_message(
                                                                    true.into(),
                                                                    "Please provide token".to_owned().into(),
                                                                ),
                                                            ]
                                                        />
                                                    </Field>
                                                }
                                                    .into_any()
                                            }
                                            _ => {
                                                view! {
                                                    <Field label="Perplexity API Token" required=true>
                                                        <Input
                                                            class="w-full"
                                                            value=llm_token
                                                            disabled=loading
                                                            rules=vec![
                                                                InputRule::required_with_message(
                                                                    true.into(),
                                                                    "Please provide token".to_owned().into(),
                                                                ),
                                                            ]
                                                        />
                                                    </Field>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </div>
                                    <Divider />
                                    <div id="recipes_source" class="p-1 mt-4 text-sm w-full gap-1">
                                        <Field label="Store in NextCloud">
                                            <Switch checked=cloud_storage />
                                        </Field>
                                    </div>
                                    <Show when=cloud_storage>
                                        <div id="cloud_uri" class="p-1 mt-4 text-sm w-full">
                                            <Field label="Nextcloud URI" required=true>
                                                <Input
                                                    class="w-full"
                                                    rules=vec![
                                                        InputRule::required_with_message(
                                                            cloud_storage.get().into(),
                                                            "Please provide URI".to_owned().into(),
                                                        ),
                                                    ]
                                                    value=cloud_uri
                                                    disabled=loading
                                                />
                                            // invalid=cloud_uri_invalid
                                            </Field>
                                        </div>
                                        <div id="cloud_username" class="p-1 mt-4 text-sm w-full">
                                            <Field label="Nextcloud username" required=true>
                                                <Input
                                                    class="w-full"
                                                    rules=vec![
                                                        InputRule::required_with_message(
                                                            cloud_storage.get().into(),
                                                            "Please provide username".to_owned().into(),
                                                        ),
                                                    ]

                                                    value=cloud_username
                                                    disabled=loading
                                                />
                                            // invalid=cloud_username_invalid
                                            </Field>
                                        </div>
                                        <div id="cloud_pass" class="p-1 mt-4 text-sm w-full">
                                            <Field label="Nextcloud password" required=true>
                                                <Input
                                                    class="w-full"
                                                    input_type=InputType::Password
                                                    rules=vec![
                                                        InputRule::required_with_message(
                                                            cloud_storage.get().into(),
                                                            "Please provide password".to_owned().into(),
                                                        ),
                                                    ]
                                                    value=cloud_pass
                                                    disabled=loading
                                                />
                                            // invalid=cloud_pass_invalid
                                            </Field>
                                        </div>
                                    </Show>
                                </div>
                                <div class="grow"></div>
                                <Button
                                    on:click={
                                        let field_context = FieldContextInjection::expect_context();
                                        move |_e: leptos::ev::MouseEvent| {
                                            loading.set(true);
                                            if field_context.validate() {
                                                spawn_local(async move {
                                                    let args = to_value(
                                                            &Args {
                                                                config: Config {
                                                                    llm: llm_service.get_untracked().into(),
                                                                    ai_token: llm_token.get_untracked(),
                                                                    cloud_uri: cloud_uri.get_untracked(),
                                                                    cloud_username: cloud_username.get_untracked(),
                                                                    cloud_pass: cloud_pass.get_untracked(),
                                                                    recipes_source: match cloud_storage.get_untracked() {
                                                                        true => RecipesSource::Cloud,
                                                                        false => RecipesSource::Local,
                                                                    },
                                                                    ..Default::default()
                                                                },
                                                            },
                                                        )
                                                        .unwrap();
                                                    match invoke("save_config", args).await {
                                                        Ok(_) => {
                                                            toaster
                                                                .dispatch_toast(
                                                                    move || {
                                                                        view! {
                                                                            <Toast>
                                                                                <ToastTitle>"Saved"</ToastTitle>
                                                                            </Toast>
                                                                        }
                                                                    },
                                                                    ToastOptions::default()
                                                                        .with_intent(ToastIntent::Success)
                                                                        .with_position(ToastPosition::Top),
                                                                );
                                                            navigate.get_untracked()("/", Default::default());
                                                        }
                                                        Err(err) => {
                                                            loading.set(false);
                                                            command_error
                                                                .set(Some(from_value::<CommandError>(err).unwrap()))
                                                        }
                                                    };
                                                });
                                            } else {
                                                loading.set(false);
                                            }
                                        }
                                    }
                                    disabled=loading
                                    appearance=ButtonAppearance::Primary
                                    class="m-4"
                                >
                                    Save
                                </Button>
                            </div>
                        </FieldContextProvider>
                    }
                        .into_any()
                } else {
                    view! {
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        // invalid=cloud_uri_invalid

                        // invalid=cloud_username_invalid
                        // invalid=cloud_pass_invalid
                        <div class="flex flex-col h-full justify-center">
                            <Spinner />
                        </div>
                    }
                        .into_any()
                }
            }}

        </main>
    }
}
