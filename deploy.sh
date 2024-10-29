rm -rf build

mkdir -p build
mkdir -p build/backend
mkdir -p build/backend/database
mkdir -p build/frontend

cd backend
cargo build --release --target=aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/backend ../build/backend/nnn-server
cp -r database/ ../build/backend/
cp .env ../build/backend/
cd ..

cd frontend
npx vite build
cp -r dist ../build/frontend/dist
cd ..

cd build/backend/database
sh dbreset.sh
cd ../../..

echo "pkill server" | ssh 192.168.1.29

cd build
scp -r * 192.168.1.29:NNN-With-Friends

echo "cd NNN-With-Friends/backend; ./nnn-server" | ssh 192.168.1.29
