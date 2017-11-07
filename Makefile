publish:
	cat README-header.md > README.md
	cargo readme --no-title >> README.md
	cargo publish

.PHONY: publish
