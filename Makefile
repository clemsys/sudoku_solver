all: build copy

build:
	cargo build --release

copy:
	cp target/release/solver .

clean:
	cargo clean
	rm solver