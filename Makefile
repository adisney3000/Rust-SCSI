default:
	cargo build --release
	cbindgen --config cbindgen.toml --crate scsi --output scsi.h
	gcc -std=c99 -O2 -o simple_example simple_example.c -I. -L./target/release -lscsi

run:
	LD_LIBRARY_PATH=./target/release ./simple_example /dev/tape_location/stk_XYZZY_A1
	target/release/simple_example -d /dev/tape_location/stk_XYZZY_A1

docs:
	cargo doc --no-deps --release

test:
	cargo test --release

clean:
	cargo clean
	rm simple_example scsi.h
