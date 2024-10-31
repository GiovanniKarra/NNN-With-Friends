#!/bin/env sh

rm -rf build

mkdir -p build
mkdir -p build/backend
mkdir -p build/database
mkdir -p build/frontend

cd backend || exit
cargo build --release --target=aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/backend ../build/backend/nnn-server
cp .env ../build/backend/
cd ..

cp -r database/ build/

cd frontend || exit
npx vite build
cp -r dist ../build/frontend/dist
cd ..

cd build/database || exit
sh dbreset.sh
cd ..

TO_COPY=frontend
if [ "$1" -ge 1 ]; then
	echo "pkill server" | ssh 192.168.1.29
	TO_COPY="$TO_COPY backend"
fi
if [ "$2" -ge 1 ]; then
	TO_COPY="$TO_COPY database"
fi
scp -r $TO_COPY 192.168.1.29:NNN-With-Friends

if [ "$1" -ge 1 ]; then
	echo "cd NNN-With-Friends/backend; ./nnn-server" | ssh 192.168.1.29
fi
