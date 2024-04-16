use serde_json::json;
use thirtyfour::common::capabilities::desiredcapabilities::CapabilitiesHelper;
use thirtyfour::prelude::*;

fn cleanup() {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    for p in system.processes_by_name("tauri") {
        if let Some(exe) = p.exe() {
            if exe.to_str().unwrap().contains("tauri-leptos-template") {
                p.kill();
            }
        }
    }
}

#[tokio::test]
async fn test_results() -> WebDriverResult<()> {
    let tauri_driver = std::process::Command::new("tauri-driver")
        .spawn()
        .expect("Failed to start tauri-driver");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut caps = Capabilities::new();
    caps.add(
        "tauri:options",
        json!({ "application": "./target/x86_64-unknown-linux-gnu/release/tauri-leptos-template" }),
    )?;
    caps.add("browserName", "wry")?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto("tauri://localhost/").await?;

    let elem = driver
        .query(By::XPath("//p[text()[contains(., 'Ok Response')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;
    let button = driver.find(By::Tag("button")).await?;
    button.click().await?;
    let elem = driver
        .query(By::XPath("//p[text()[contains(., 'Error Response')]]"))
        .first()
        .await?;
    elem.wait_until().displayed().await?;

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
