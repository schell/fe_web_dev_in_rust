#! /bin/bash

if [ -z ${GITHUB_REF+x} ]; then
    export GITHUB_REF=`git rev-parse --symbolic-full-name HEAD`
fi

export PATH=$PATH:$HOME/.cargo/bin

if hash rustup 2>/dev/null; then
    echo "Have rustup, skipping installation..."
else
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "  done installing the rust toolchain."
fi

if hash wasm-pack 2>/dev/null; then
    echo "Have wasm-pack, skipping installation..."
else
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    echo "  done installing wasm-pack."
fi

if hash mdbook 2>/dev/null; then
    echo "Have mdbook, skipping installation..."
else
    echo "Installing mdbook..."
    cargo install mdbook
    echo "  done installing mdbook."
fi

if hash jq 2>/dev/null; then
    echo "Have jq, skipping installation..."
else
    echo "Installing jq"
    wget https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64
    chmod 755 jq-linux64
    mv jq-linux64 $HOME/cargo/bin/jq
    echo "  done installing jq."
fi

echo "Building the book"
mdbook build

echo "Building web-sys-examples w/ wasm-pack..."
mkdir -p book/web-sys-examples
wasm-pack build --release --target no-modules web-sys-examples || exit 1
cp -R web-sys-examples/pkg book/web-sys-examples/
cp web-sys-examples/index.html book/web-sys-examples/index.html
cp web-sys-examples/style.css book/web-sys-examples/style.css
mv book intro-to-rust-web || exit 1
sleep 1
tar czvf intro-to-rust-web.tar.gz intro-to-rust-web || exit 1
sleep 1
ls -lah intro-to-rust-web.tar.gz
echo "Done building on ${GITHUB_REF}"
