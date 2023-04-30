lint:
	cargo clippy
fix:
	cargo clippy --fix

test:
	cargo test --all-features --color=auto -v

test-doc:
	cargo test --doc --all-features --color=auto -v

badges:
	python scripts/make_badges.py NativeUI
