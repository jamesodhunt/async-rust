IN_FILE = async-rust.md
OUT_FILE = /tmp/async-rust.html

OUT_FMT = slidy

default: view

build:
	pandoc \
		--standalone \
		-f markdown \
		-t "$(OUT_FMT)" \
		-i \
		-o "$(OUT_FILE)" "$(IN_FILE)"

view: build
	xdg-open "file://$(OUT_FILE)"
