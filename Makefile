VERSION_FILE := "Cargo.toml"

install:
	cargo install --path .

publish:
	sed -i -r "s/0\.0\.0/${VERSION}/g" "$(VERSION_FILE)" \
	&& cargo publish --allow-dirty \
