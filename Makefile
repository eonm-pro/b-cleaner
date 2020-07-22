python-lib-linux:
	cargo build --release --lib --features=python,html
	rm -f target/release/b_cleaner.so
	cp target/release/libb_cleaner.so target/release/b_cleaner.so

python-lib-windows:
	cargo build --lib --release --target=x86_64-pc-windows-gnu --features=python,html
	rm -f target/release/b_cleaner.dll
	cp -f target/release/libb_cleaner.dll target/release/b_cleaner.dll

python-lib: python-lib-windows python-lib-linux

test:
	cargo test
	cargo test --doc

build-doc:
	cargo doc --lib --no-deps --features=stem,html

open-doc:
	cargo doc --open --lib --no-deps --features=stem,html
	