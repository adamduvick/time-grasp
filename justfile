# https://just.systems

ios_dev:
    cargo tauri ios dev

ios_reload_proj:
    xcodegen generate -s src-tauri/gen/apple/project.yml -p src-tauri/gen/apple

ios_cleanup:
    rm -rf /Users/adamduvick/Library/Developer/Xcode/DerivedData/time-grasp*
