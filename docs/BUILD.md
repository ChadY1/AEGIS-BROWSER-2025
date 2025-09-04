# Build rapide

## OpenSearch
docker compose -f docker/opensearch-compose.yml up -d

## API
cd search-api && cargo run

## Desktop
cd desktop/ui && npm i && npm run build
cd ../src-tauri && cargo tauri build

## Mobile
cd mobile && flutter pub get
flutter run -d android
