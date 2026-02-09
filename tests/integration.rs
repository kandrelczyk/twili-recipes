use std::process::Child;

use serde_json::json;
use serial_test::serial;
use thirtyfour::prelude::*;

async fn cleanup(driver: &WebDriver, tauri_driver: Child) -> Result<(), WebDriverError> {
    driver.clone().quit().await?;
    // tauri_driver.kill() sends SIGKILL which is not handled by tauri driver (it will not
    // terminate WebKitWebDriver)
    let mut kill = std::process::Command::new("kill")
        .args(["-2", &tauri_driver.id().to_string()])
        .spawn()?;
    kill.wait()?;

    // our app wil not be cloused automatically by WebKitWeDriver for some reason
    let mut child = std::process::Command::new("rm")
        .arg(
            format!(
                "{}/{}",
                std::env::var("HOME").unwrap(),
                ".local/share/net.curiana.recipes/test.cfg"
            )
            .as_str(),
        )
        .spawn()
        .expect("Failed to remove settings");
    child.wait().expect("Failed to wait for child");
    let mut child = std::process::Command::new("rm")
        .arg(
            format!(
                "{}/{}",
                std::env::var("HOME").unwrap(),
                ".local/share/net.curiana.recipes/test.recipes"
            )
            .as_str(),
        )
        .spawn()
        .expect("Failed to remove recipes");
    child.wait().expect("Failed to wait for child");
    let mut system = sysinfo::System::new();
    system.refresh_all();
    for p in system.processes_by_name("twili") {
        if let Some(exe) = p.exe() {
            if exe.to_str().unwrap().contains("twili-recipes") {
                p.kill();
            }
        }
    }

    Ok(())
}

async fn populate_config(driver: &WebDriver, host: String) -> Result<(), WebDriverError> {
    let elem = driver
        .query(By::XPath("//label[text()[contains(., 'OpenAI')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    elem.click().await?;
    let elem = driver.query(By::Id("gpt_api_token")).first().await?;
    elem.wait_until().displayed().await?;
    elem.send_keys("gpt_api_token").await?;

    let elem = driver
        .query(By::ClassName("thaw-switch"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.click().await?;

    send_keys_by_id(driver, "cloud_uri", &format!("{}{}", "http://", host)).await?;
    send_keys_by_id(driver, "cloud_username", "username").await?;
    send_keys_by_id(driver, "cloud_pass", "password").await?;

    Ok(())
}

async fn setup() -> (Child, WebDriver) {
    let tauri_driver = std::process::Command::new("tauri-driver")
        .spawn()
        .expect("Failed to start tauri-driver");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut caps = DesiredCapabilities::chrome();
    caps.insert_base_capability(
        "tauri:options".to_owned(),
        json!({ "application": "./target/x86_64-unknown-linux-gnu/release/twili-recipes", "args": ["-c", "test.cfg", "-r", "test.recipes"] }),
    );
    caps.insert_base_capability("browserName".to_owned(), json!("wry"));

    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .expect("Failed to create driver");

    (tauri_driver, driver)
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_initial_setup() -> WebDriverResult<()> {
    let (tauri_driver, driver) = setup().await;

    let mut server = mockito::Server::new_async().await;

    let get_list = server
        .mock(
            "GET",
            "/remote.php/dav/files/username/.TwiliRecipes/.list.json",
        )
        .with_body("[]")
        .create_async()
        .await;

    let get_list_full = server
        .mock(
            "GET",
            "/remote.php/dav/files/username/.TwiliRecipes/.list.json",
        )
        .with_body(r#"[{"name":"Szarlotka", "filename":"some_file_name"}]"#)
        .create_async()
        .await;

    driver.goto("tauri://localhost/").await?;

    let elem = driver
        .query(By::XPath("//div[text()[contains(., 'Initial setup')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    populate_config(&driver, server.host_with_port()).await?;

    click_by_xpath(&driver, "//button[text()[contains(., 'Save')]]").await?;

    let elem = driver
        .query(By::XPath(
            "//p[text()[contains(., 'have any recipes yet')]]",
        ))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    driver
        .query(By::Tag("button"))
        .first()
        .await?
        .click()
        .await?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    let refresh = driver.query(By::Id("refresh")).first().await?;
    refresh.wait_until().displayed().await?;
    refresh.click().await?;

    let elem = driver
        .query(By::XPath("//div[text()[contains(., 'Szarlotka')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    elem.wait_until().enabled().await?;

    get_list.assert_async().await;
    get_list_full.assert_async().await;

    cleanup(&driver, tauri_driver).await?;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_init_recipes_list() -> WebDriverResult<()> {
    let (tauri_driver, driver) = setup().await;

    let mut server = mockito::Server::new_async().await;

    let get_list = server
        .mock(
            "GET",
            "/remote.php/dav/files/username/.TwiliRecipes/.list.json",
        )
        .with_status(404)
        .expect(4)
        .create_async()
        .await;

    let init_dir = server
        .mock("MKCOL", "/remote.php/dav/files/username/.TwiliRecipes")
        .with_status(200)
        .create_async()
        .await;

    let init_list = server
        .mock(
            "PUT",
            "/remote.php/dav/files/username/.TwiliRecipes/.list.json",
        )
        .with_status(200)
        .create_async()
        .await;

    driver.goto("tauri://localhost/").await?;

    let elem = driver
        .query(By::XPath("//div[text()[contains(., 'Initial setup')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    populate_config(&driver, server.host_with_port()).await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Save')]]").await?;

    let elem = driver
        .query(By::XPath("//p[text()[contains(., 'any recipes')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    get_list.assert_async().await;
    init_dir.assert_async().await;
    init_list.assert_async().await;

    cleanup(&driver, tauri_driver).await?;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_error_when_initializing_recipes() -> WebDriverResult<()> {
    let (tauri_driver, driver) = setup().await;

    let mut server = mockito::Server::new_async().await;

    let get_list = server
        .mock(
            "GET",
            "/remote.php/dav/files/username/.TwiliRecipes/.list.json",
        )
        .with_status(404)
        .expect(4)
        .create_async()
        .await;

    let init_dir = server
        .mock("MKCOL", "/remote.php/dav/files/username/.TwiliRecipes")
        .with_status(400)
        .create_async()
        .await;

    driver.goto("tauri://localhost/").await?;

    let elem = driver
        .query(By::XPath("//div[text()[contains(., 'Initial setup')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    populate_config(&driver, server.host_with_port()).await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Save')]]").await?;

    let elem = driver
        .query(By::XPath("//p[text()[contains(., 'Failed to load')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    get_list.assert_async().await;
    init_dir.assert_async().await;

    cleanup(&driver, tauri_driver).await?;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_local_recipe_storage() -> WebDriverResult<()> {
    let (tauri_driver, driver) = setup().await;

    driver.goto("tauri://localhost/").await?;

    let elem = driver
        .query(By::XPath("//div[text()[contains(., 'Initial setup')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    click_by_xpath(&driver, "//button[text()[contains(., 'Save')]]").await?;

    let elem = driver
        .query(By::XPath("//p[text()[contains(., 'any recipes yet')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

    click_by_class(&driver, "fab").await?;

    let elem = driver
        .query(By::XPath("//button[text()[contains(., 'Add manually')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    elem.click().await?;

    let elem = driver
        .query(By::XPath("//button[text()[contains(., 'Add group')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Add group')]]").await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Add step')]]").await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Add ingredient')]]").await?;
    let inputs = driver.query(By::Tag("input")).all_from_selector().await?;
    inputs.first().unwrap().send_keys("Recipe").await?;
    inputs.get(1).unwrap().send_keys("Group1").await?;
    inputs.get(2).unwrap().send_keys("Ingredient").await?;
    inputs.get(3).unwrap().send_keys("10").await?;
    inputs.get(4).unwrap().send_keys("kg").await?;
    inputs.get(5).unwrap().send_keys("5").await?;
    driver
        .query(By::Tag("textarea"))
        .first()
        .await?
        .send_keys("step text")
        .await?;
    click_by_xpath(&driver, "//button[text()[contains(., 'Save')]]").await?;
    click_by_class(&driver, "thaw-card").await?;
    let texts = vec!["Size", "Recipe", "GROUP1"];
    for t in texts {
        driver
            .query(By::XPath(format!("//div[text()[contains(., '{t}')]]")))
            .first()
            .await?
            .wait_until()
            .displayed()
            .await?;
    }
    click_by_class(&driver, "thaw-menu-trigger").await?;
    click_by_xpath(&driver, "//span[text()[contains(., 'Edit JSON')]]").await?;
    let texarea = driver.query(By::Tag("textarea")).first().await?;
    texarea.send_keys(",").await?;
    let save_buttom = driver
        .query(By::XPath("//button[text()[contains(., 'Save')]]"))
        .first()
        .await?;
    save_buttom.click().await?;
    driver
        .query(By::XPath("//div[text()[contains(., 'Enter valid JSON')]]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;
    texarea.send_keys(Key::Backspace).await?;
    save_buttom.click().await?;
    click_by_class(&driver, "thaw-card").await?;
    click_by_class(&driver, "thaw-menu-trigger").await?;
    click_by_xpath(&driver, "//span[text()[contains(., 'Delete')]]").await?;
    click_by_xpath(&driver, "//Button[text()[contains(., 'Delete')]]").await?;
    driver
        .query(By::XPath("//p[text()[contains(., 'any recipes yet')]]"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    cleanup(&driver, tauri_driver).await?;
    Ok(())
}

async fn click_by_xpath(driver: &WebDriver, path: &str) -> Result<(), WebDriverError> {
    driver.query(By::XPath(path)).first().await?.click().await?;
    Ok(())
}

async fn click_by_class(driver: &WebDriver, path: &str) -> Result<(), WebDriverError> {
    driver
        .query(By::ClassName(path))
        .first()
        .await?
        .click()
        .await?;
    Ok(())
}

async fn send_keys_by_id(driver: &WebDriver, path: &str, text: &str) -> Result<(), WebDriverError> {
    driver
        .query(By::Id(path))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?
        .send_keys(text)
        .await?;
    Ok(())
}
