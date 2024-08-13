use std::process::Child;

use serde_json::json;
use thirtyfour::common::capabilities::desiredcapabilities::CapabilitiesHelper;
use thirtyfour::prelude::*;

fn cleanup() {
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
}

async fn setup() -> (Child, WebDriver) {
    let tauri_driver = std::process::Command::new("tauri-driver")
        .spawn()
        .expect("Failed to start tauri-driver");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut caps = Capabilities::new();
    caps.add(
        "tauri:options",
        json!({ "application": "./target/x86_64-unknown-linux-gnu/release/twili-recipes", "args": ["-c", "test.cfg"] }),
    ).expect("Failed to set capabilities");
    caps.add("browserName", "wry")
        .expect("Failed to set capabilities");

    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .expect("Failed to create driver");

    (tauri_driver, driver)
}

#[tokio::test]
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

    driver
        .query(By::XPath("//button[text()[contains(., 'Save')]]"))
        .first()
        .await?
        .click()
        .await?;

    let elem = driver
        .query(By::Id("api_token"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.wait_until().displayed().await?;
    elem.send_keys("gpt_api_token").await?;

    let elem = driver
        .query(By::Id("recipes_source"))
        .first()
        .await?
        .find(By::Tag("div"))
        .await?;
    elem.wait_until().displayed().await?;
    elem.click().await?;

    let elem = driver
        .query(By::Id("cloud_uri"))
        .first()
        .await?
        .find(By::Tag("input"))
        .await?;
    elem.send_keys(format!("{}{}", "http://", server.host_with_port()))
        .await?;

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

    // tauri_driver.kill() sends SIGKILL which is not handled by tauri driver (it will not
    // terminate WebKitWebDriver)
    let mut kill = std::process::Command::new("kill")
        .args(["-2", &tauri_driver.id().to_string()])
        .spawn()?;
    kill.wait()?;

    // our app wil not be cloused automatically by WebKitWeDriver for some reason
    cleanup();
    Ok(())
}
