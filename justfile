outputDir := './dist'
zipFile := "ita_dashboard.zip"
releaseDir := './target/release'
winReleaseDir := './target/x86_64-pc-windows-gnu/release'

# build for release
build:
    cargo build --release
    mkdir -p "{{outputDir}}"
    cp "{{releaseDir}}/ita_dashboard" "{{outputDir}}/"
    cp db_config.toml.example {{outputDir}} 
    cp icon.png {{outputDir}} 
    (cd "{{outputDir}}" && zip -r "{{zipFile}}" .)

build_win:
    cargo build --target x86_64-pc-windows-gnu --release
    mkdir -p "{{outputDir}}"
    cp "{{winReleaseDir}}/ita_dashboard.exe" "{{outputDir}}/"
    cp db_config.toml.example {{outputDir}} 
    cp icon.png {{outputDir}} 
    (cd "{{outputDir}}" && zip -r "{{zipFile}}" .)

# run the application
run:
    cargo run

# check the code
check:
    cargo check

# clean zip and dist artifacts
clean_artifacts:
    (Remove-Item -Recurse -Force {{outputDir}} -ErrorAction SilentlyContinue) -and (exit 0) 
    (Remove-Item -Recurse -Force {{zipFile}} -ErrorAction SilentlyContinue) -and (exit 0)

# full cleanup including cargo 
cleanup: clean_artifacts
    cargo clean