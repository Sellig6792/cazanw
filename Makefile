

build:
	wasm-pack build cazanw --target web --out-dir ../pkg $(RELEASE)

	cross build --bins --target x86_64-unknown-linux-gnu  $(RELEASE)
	cross build --bins --target aarch64-unknown-linux-gnu $(RELEASE)

	cross build --bins --target x86_64-pc-windows-gnu  $(RELEASE)
	cross build --bins --target aarch64-pc-windows-msvc  $(RELEASE)

	cross build --bins --target x86_64-apple-darwin  $(RELEASE)
	cross build --bins --target aarch64-apple-darwin  $(RELEASE)
	cargo run -p post-build --target-dir target/post-build $(RELEASE)



clean:
	rm -rf pkg
	cargo clean

clean-pkg:
	rm -rf pkg