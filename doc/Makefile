IN_FILE = async-rust.md
OUT_FILE = /tmp/async-rust.html

OUT_FMT = slidy

default: view

build:
	pandoc \
		--standalone \
		--metadata title="Async rust overview" \
		--metadata author="James O. D. Hunt" \
		--metadata date="Presented 7 October 2022" \
		-f markdown \
		-t "$(OUT_FMT)" \
		-i \
		-o "$(OUT_FILE)" "$(IN_FILE)"

view: build
	xdg-open "file://$(OUT_FILE)"
