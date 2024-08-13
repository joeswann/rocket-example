.PHONY: build run clean

build:
	cargo build --release

install:
	sudo cp rust-rocket.service /etc/systemd/system/rust-rocket.service \
	&& sudo systemctl daemon-reload
	&& sudo systemctl enable rust-rocket.service
	&e sudo systemctl start rust-rocket.service

run:
	cargo run

clean:
	cargo clean
