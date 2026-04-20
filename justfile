set shell := ["powershell.exe", "-c"]

outputDir := '.\dist'
zipFile := "ita_dashboard.zip"
releaseDir := '.\target\release'

# build for release
build:
    cargo build --release
    New-Item -ItemType Directory -Force -Path {{outputDir}} | Out-Null
    Get-ChildItem "{{releaseDir}}\*.exe" | Select-Object -First 1 | Copy-Item -Destination {{outputDir}}
    Copy-Item db_config.toml.example {{outputDir}} 
    Copy-Item icon.png {{outputDir}} 
    Compress-Archive -Path "{{outputDir}}\*" -DestinationPath {{zipFile}} -Force

# run the application
run:
    cargo run

# check the code
check:
    cargo check