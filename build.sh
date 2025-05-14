source ./web/rust_flags.sh

cd web 
dx bundle --release --platform web
cd ../

git checkout --orphan static
git rm -rf .
cp -r ./target/dx/web/release/web/public/* .

git add index.html 
git add assets/*
git add wasm/*

git commit -m "test"

git clean -fdx
