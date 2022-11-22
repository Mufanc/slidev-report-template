.PHONY : all init clean

all:
	mkdir -p dist
	pnpm run export --output dist/slides-export.pdf
	pnpm run export --with-clicks --output dist/slides-export-clicks.pdf
	poetry run pdf2pptx dist/slides-export-clicks.pdf -o dist/export.pptx
	pnpm run build -o dist/dist
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	zip -r slides-export.zip dist/* target/*/release/server* slides.md
	zip --delete slides-export.zip target/*/release/server.d 

init:
	pnpm add -D playwright-chromium
	poetry install
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-pc-windows-gnu

clean:
	cargo clean
	rm -rf dist
	rm -f *.zip


