use std::fs;
use std::path::PathBuf;

fn open_package_json() -> serde_json::Map<String, serde_json::Value> {
    let package_json =
        fs::read_to_string("./pkg/package.json").expect("Failed to read package.json");
    serde_json::from_str(&package_json).expect("Failed to parse package.json")
}

fn main() {
    // Create bin directory in pkg
    fs::create_dir_all("./pkg/bin").expect("Failed to create bin directory");

    // Put the postinstall.js script in bin directory
    fs::copy("./post-build/src/cazanw.js", "./pkg/bin/cazanw.js")
        .expect("Failed to copy cazanw.js");

    // Copy built files to bin directory (from target/

    let mut package_json = open_package_json();

    package_json.insert(
        String::from("bin"),
        serde_json::Value::String(String::from("./bin/cazanw.js")),
    );

    // let mut scripts = serde_json::Map::new();
    // scripts.insert(String::from("postinstall"), serde_json::Value::String(String::from("node ./bin/postinstall.js")));
    // package_json.insert(String::from("scripts"), serde_json::Value::Object(scripts));

    fs::write(
        "./pkg/package.json",
        serde_json::to_string_pretty(&package_json).expect("Failed to convert to string"),
    )
    .expect("Failed to write package.json");

    // Move copy all "target" dirs to pkg/bin. To know if it is a target dir check if there is a files that begins with the scripts' names
    let main_target_dir = fs::read_dir("./target").expect("Failed to read target directory");

    let target_dirs: Vec<PathBuf> = main_target_dir
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|entry| entry.path())
        .collect();

    for target_dir in target_dirs {
        // Script may finish with .exe if it is a windows target
        let script_name = format!(
            "cazanw{}",
            if target_dir.to_str().unwrap().contains("windows") {
                ".exe"
            } else {
                ""
            }
        );

        let debug_or_release = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        let script_path = target_dir.join(debug_or_release).join(script_name.as_str());

        if script_path.exists() {
            let potential_target = target_dir.file_name().unwrap().to_str().unwrap();
            // Create target dir in pkg/bin
            fs::create_dir_all(format!("./pkg/bin/{}", potential_target))
                .expect("Failed to create target directory");

            let dest_path = format!("./pkg/bin/{}/{}", potential_target, script_name.as_str());
            fs::copy(&script_path, &dest_path).expect("Failed to copy file");
        }
    }
}
