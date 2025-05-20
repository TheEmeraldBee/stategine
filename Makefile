test:
	cargo test

ascii-forge-example:
	cargo run --example ascii-forge

countdown-example:
	cargo run --example countdown

ratatui-example:
	cargo run --example ratatui

VERSION_FILE := Cargo.toml

publish:
	sed -i -r "s/version=\"0\.0\.0\"/version=\"${VERSION}\"/g" $(VERSION_FILE) \
	  && cargo publish --allow-dirty

