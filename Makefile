publish:
	cat README-header.md > README.md
	cargo readme --no-title >> README.md
	cargo package
	cargo publish

.PHONY: publish
