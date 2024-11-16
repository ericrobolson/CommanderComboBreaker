run:
	cargo watch -w src -x run

card_finder:
	cargo watch -w src -x 'run -- Magus Lucea Kane'

test:
	cargo watch -w src -x test
