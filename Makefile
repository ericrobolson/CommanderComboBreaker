run:
	cargo watch -w src -x run

card_finder:
	cargo watch -w src -x 'run -- Staff of Domination'

test:
	cargo watch -w src -x test

db:
	sudo docker-compose up -d

install:
	cargo install cargo-watch