# Install Mise

https://mise.jdx.dev/getting-started.html

# install gcc and such
sudo apt install build-essential

# run
just run

# build 
just build
just build_win

# set up build under Linux for Windows target
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
sudo apt-get install zip

sudo apt install libaio1 unzip