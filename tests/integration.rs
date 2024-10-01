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
    std::process::Command::new("rm")
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

    let elem = driver
        .query(By::Id("cloud_uri"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.send_keys(format!("{}{}", "http://", host)).await?;

    let elem = driver
        .query(By::Id("cloud_username"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.send_keys("username").await?;

    let elem = driver
        .query(By::Id("cloud_pass"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.send_keys("password").await?;

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
        json!({ "application": "./target/x86_64-unknown-linux-gnu/release/twili-recipes", "args": ["-c", "test.cfg"] }),
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

    driver
        .query(By::XPath("//button[text()[contains(., 'Save')]]"))
        .first()
        .await?
        .click()
        .await?;

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
    driver
        .query(By::XPath("//button[text()[contains(., 'Save')]]"))
        .first()
        .await?
        .click()
        .await?;

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
    driver
        .query(By::XPath("//button[text()[contains(., 'Save')]]"))
        .first()
        .await?
        .click()
        .await?;

    let elem = driver
        .query(By::XPath("//span[text()[contains(., 'Failed to load')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    get_list.assert_async().await;
    init_dir.assert_async().await;

    cleanup(&driver, tauri_driver).await?;
    Ok(())
}
