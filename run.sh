mkdir sources
cd sources
# Clone the repositories
git clone https://github.com/Pumpkin-MC/Pumpkin.git
git clone https://github.com/FabricMC/yarn.git

cd yarn
# Decompile the yarn mappings
./gradlew decompileVineFlower

cd ..

cargo run --release

cd sources

rm -rf Pumpkin/
rm -rf yarn/
