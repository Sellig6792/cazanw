build:
	wasm-pack build --target web --out-dir ../pkg $(RELEASE)

	cross build -p cazanw-bin --target x86_64-unknown-linux-gnu  $(RELEASE)
	cross build -p cazanw-bin --target aarch64-unknown-linux-gnu $(RELEASE)

	cross build -p cazanw-bin --target x86_64-pc-windows-gnu  $(RELEASE)
	cross build -p cazanw-bin --target aarch64-pc-windows-msvc  $(RELEASE)

	cross build -p cazanw-bin --target x86_64-apple-darwin  $(RELEASE)
	cross build -p cazanw-bin --target aarch64-apple-darwin  $(RELEASE)
	cargo run -p post-build --target-dir target/post-build $(RELEASE)

clean:
	rm -rf pkg
	cargo clean

wclean:
	rm -rf pkg
	wargo clean

clean-pkg:
	rm -rf pkg
