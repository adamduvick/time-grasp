# https://just.systems
set dotenv-load

dev:
    cargo tauri dev

ios_dev:
    cargo tauri ios dev --open

ios_reload_proj:
    xcodegen generate -s src-tauri/gen/apple/project.yml -p src-tauri/gen/apple

ios_cleanup:
    rm -rf ~/Library/Developer/Xcode/DerivedData/time-grasp*

[working-directory('src-backend')]
reload_database:
    rm -rf .sqlx "$DATABASE_PATH"
    cargo sqlx db create && cargo sqlx migrate run && cargo sqlx prepare

test_backend_model:
    cargo test --package time-grasp model:: -- --test-threads=1

test:
    cargo test --workspace --all-targets
