use confy;
use recipes_common::Config;
use tauri::async_runtime::Mutex;

use crate::{
    ai::{AIClient, ChatGTPClient},
    commands::error::CommandError,
    recipes::{ncclient::NCClient, RecipesProvider},
};

#[tauri::command]
pub async fn command(
    error: bool,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
) -> Result<String, CommandError> {
    let m = manager.lock().await;

    println!("{:?}", m.as_ref().unwrap().list_recipes().await.unwrap());

    let ai = ai_client.lock().await;
    let recipe = ai.as_ref()
        .unwrap()
        .parse_recipe(r#"
Ingredientes
Para 6 personas

    Galletas tipo Digestive 16
    Queso crema 400
    Mantequilla 120 g
    Nata líquida 200 ml
    Huevos 4
    Harina de repostería 20 g
    Azúcar 80 g
    Ralladura de limón 10 g

Cómo hacer la receta clásica de la tarta de queso
Dificultad: Fácil

    Tiempo total 45 m
    Elaboración 10 m
    Cocción 35 m
    Reposo 30 m

Prepara la base mezclando la mantequilla con la harina de galletas y cubre con ella la base de la fuente donde vayas a preparar la tarta. Aprieta bien para que quede compacto y mételo en la nevera para que endurezca al solidificarse la mantequilla.

Mientras enfría la base, precalienta el horno a 200ºC. De este modo, tendrás tiempo de preparar el relleno mientras el horno está listo. Como la preparación del relleno es rápida y sencilla, cuando lo tengas listo tendrás también preparados el horno y la base de tu tarta de queso.

Para hacer la crema o el relleno de la tarta, batimos con las varillas los huevos con el azúcar hasta que blanqueen. Después agregamos la nata líquida, las dos tarrinas de queso crema y mezclamos bien. Añadimos la harina y la ralladura de limón y una vez esté homogéneo y sin grumos, lo vertemos sobre la base de galletas.

Horneamos bajando la temperatura a 170-180ºC y tendremos la tarta de queso lista tras unos 30 minutos de cocción. Para comprobar el punto puedes meter una brocheta de madera en la zona central y si sale limpio o casi limpio, puedes apagar y dejar que se enfríe hasta estar cuajada pero no dura. Ten en cuenta que también se endurecera un poco al refrigerarse en la nevera.
                      "#.to_owned())
        .await
        .unwrap();

    println!("{:?}", recipe);

    std::thread::sleep(std::time::Duration::from_secs(1));
    if !error {
        Ok("Ok Response".to_owned())
    } else {
        Err(CommandError {
            reason: "Error Response".to_owned(),
        })
    }
}

#[tauri::command]
pub async fn initialize(
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
) -> Result<bool, CommandError> {
    let config: Config = confy::load("twili-recipes", None)?;

    if config.all_present() {
        let mut m = manager.lock().await;
        let m2: Box<dyn RecipesProvider> = Box::new(NCClient::new(
            config.cloud_uri,
            config.cloud_username,
            config.cloud_pass,
        ));
        *m = Some(m2);

        let mut ai = ai_client.lock().await;
        let ai2: Box<dyn AIClient> =
            Box::new(ChatGTPClient::new(config.ai_token, config.ai_prompt));
        *ai = Some(ai2);

        Ok(true)
    } else {
        Ok(false)
    }
}
