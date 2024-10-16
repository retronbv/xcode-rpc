use std::process::Command;
use std::time::{SystemTime,UNIX_EPOCH};
use discord_rich_presence::{DiscordIpcClient,DiscordIpc};
use discord_rich_presence::activity;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1295848516702310501")?;
    client.connect()?;

    let started_at = activity::Timestamps::new().start(current_time());

    let payload = activity::Activity::new().timestamps(started_at.clone()).state("Idling...")
    .assets(
        activity::Assets::new()
            .large_image("project")
            .large_text("Not working on anything...")
            .small_image("xcode"),
    );
    client.set_activity(payload)?;

    let mut lastfilename = String::from("");
    loop {
        let file = current_file()?.trim().to_string();
        if !lastfilename.eq(&file) {
            let fileext = file.split(".").collect::<Vec<&str>>()[1];
            let keys = match fileext {
                "swift" => ("Swift", "swift"),
                "json" => ("JSON", "json"),
                _ => (fileext, "xcode"),
            };
            client.set_activity(activity::Activity::new()
                .timestamps(started_at.clone())
                .state(&format!("Working on {}", file))
                .assets(
                    activity::Assets::new()
                        .large_image("project")
                        .large_text(&current_project()?)
                        .small_image(keys.1)
                        .small_text(&format!("{} file",keys.0)),
                )
            )?;
            lastfilename = file;
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}

fn current_file() -> Result<String, Box<dyn std::error::Error>> {
    let file = run_osascript(
        r#"
        tell application "Xcode"
            return name of windows whose index is 1
        end tell
    "#,
    )?;
    if !file.contains(" — ") {
        return Ok(file);
    }
    let file = file.split(" — ").collect::<Vec<&str>>()[1];
    Ok(file.to_string())
}

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to obtain current time")
        .as_secs() as i64
}

fn run_osascript(script: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute command");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn current_project() -> Result<String, Box<dyn std::error::Error>> {
    let project = run_osascript(
        r#"
        tell application "Xcode"
            return active workspace document
        end tell
    "#,
    )?
    .trim()
    .to_string();
    if project == "missing value" {
        return Ok(String::new());
    }
    if project.starts_with("workspace document ") {
        return Ok(project.replace("workspace document ", ""));
    }
    Ok(project)
}