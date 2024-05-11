use std::collections::HashSet;

use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{Button, ButtonVariant, Collapse, CollapseItem, Icon, Input, Modal, TextArea};
use wasm_bindgen::prelude::*;

use crate::{components::recipes::EditRecipe, error::CommandError};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Args {
    recipe: String,
}

#[component]
pub fn AddRecipe() -> impl IntoView {
    let recipe_str = create_rw_signal("".to_owned());

    let error: RwSignal<Option<CommandError>> = create_rw_signal(None);
    let show_error = Signal::derive(move || error.get().is_some());
    let collapse = create_rw_signal(HashSet::from(["".to_string()]));
    let recipe : RwSignal<Option<String>> = create_rw_signal(Some(r#"{"ingredients":[{"name":"Galletas tipo Digestive","quantity":16,"scale":""},{"name":"Queso crema","quantity":400,"scale":""},{"name":"Mantequilla","quantity":120,"scale":"g"},{"name":"Nata líquida","quantity":200,"scale":"ml"},{"name":"Huevos","quantity":4,"scale":""},{"name":"Harina de repostería","quantity":20,"scale":"g"},{"name":"Azúcar","quantity":80,"scale":"g"},{"name":"Ralladura de limón","quantity":10,"scale":"g"}],"steps":[{"desc":"Prepara la base mezclando la Mantequilla con la Harina de Galletas y cubre con ella la base de la fuente donde vayas a preparar la tarta. Aprieta bien para que quede compacto y mételo en la nevera para que endurezca al solidificarse la Mantequilla.","time":0},{"desc":"Mientras enfría la base, precalienta el horno a 200ºC. De este modo, tendrás tiempo de preparar el relleno mientras el horno está listo. Como la preparación del relleno es rápida y sencilla, cuando lo tengas listo tendrás también preparados el horno y la base de tu tarta de queso.","time":0},{"desc":"Para hacer la crema o el relleno de la tarta, batimos con las varillas los Huevos con el Azúcar hasta que blanqueen. Después agregamos la Nata líquida, las dos tarrinas de Queso crema y mezclamos bien. Añadimos la Harina y la Ralladura de limón y una vez esté homogéneo y sin grumos, lo vertemos sobre la base de Galletas.","time":0},{"desc":"Horneamos bajando la temperatura a 170-180ºC y tendremos la tarta de queso lista tras unos 30 minutos de cocción. Para comprobar el punto puedes meter una brocheta de madera en la zona central y si sale limpio o casi limpio, puedes apagar y dejar que se enfríe hasta estar cuajada pero no dura. Ten en cuenta que también se endurecerá un poco al refrigerarse en la nevera.","time":30}]"#.to_owned()));

    let recipe_invalid = create_rw_signal(false);
    let loading = create_rw_signal(false);

    let navigate = create_rw_signal(use_navigate());

    let submit = move |_| {
        recipe_invalid.set(recipe_str.get().is_empty());

        if !recipe_invalid.get() {
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                let args = to_value(&Args {
                    recipe: recipe_str.get_untracked(),
                })
                .unwrap();
                match invoke("parse_recipe", args).await {
                    Ok(result) => {
                        recipe.set(Some(from_value(result).unwrap()));
                    }
                    Err(err) => {
                        log::info!("{:?}", err);
                        error.set(Some(from_value::<CommandError>(err).unwrap()))
                    }
                };
                loading.set(false);
            });
        }
    };

    view! {
        <Show when=move || recipe.get().is_none()
            fallback=move|| view!{<EditRecipe recipe_json=recipe.get().unwrap() go_back=move |_| recipe.set(None)/>} >
            <Modal title="LLM Error" width="300px" show=show_error>
                <p class="text-md mb-4">
                    Failed to call LLM service. Check your configuration or try again later
                </p>
                <Collapse value=collapse>
                    <CollapseItem title="Error details" key="error">
                        <p class="text-sm break-all text-wrap">
                            {move || error.get().unwrap().reason}
                        </p>
                    </CollapseItem>
                </Collapse>
            </Modal>
            <main class="flex flex-col items-center h-full"> //TODO: extract header component
                <div class="p-1 flex flex-row w-full items-center border-b border-slate-500">
                    <Button
                        class="absolute"
                        variant=ButtonVariant::Text
                        round=true
                        on:click=move |_| navigate.get()("/list", Default::default())
                    >
                        <Icon
                            width="1.5em"
                            height="1.5em"
                            icon=icondata_bi::BiChevronLeftSolid
                        />
                    </Button>
                    <div class="w-full flex flex-col items-center text-xl">
                        Add recipe
                    </div>
                </div>
                <div class="p-2 w-full max-w-xl h-full">
                    <div class="p-1 text-sm w-full h-[95%]">
                        Recipe
                        <TextArea class="h-full" value=recipe_str invalid=recipe_invalid/>
                    </div>
                </div>
                <div class="grow"></div>
                <Button on:click=submit loading class="m-4">
                    Process
                </Button>
            </main>
        </Show>
    }
}
