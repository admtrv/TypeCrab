echo "Sourcing rust_flags.sh..."
source ./web/rust_flags.sh

echo "Building the project..."
cd web 
dx bundle --release --platform web
cd ../

echo "Creating and switching to static branch..."
git branch -D static 2>/dev/null || true
git checkout --orphan static

echo "Removing existing files..."
git rm -rf .

echo "Copying build output..."
cp -r ./target/dx/web/release/web/public/* .

echo "Adding files..."
git add index.html 
git add assets/*
git add wasm/*

echo "Committing changes..."
git commit -m "Deploying static site"

echo "Pushing to origin static..."
git push origin static --force

