default: all

all: deps build

deps:
	(cd lib/rustful && make deps && make)

build:
	rustc -L lib/rustful/lib/ -o hello hello.rs

.PHONY: all deps build
