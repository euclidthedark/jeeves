run: build
	./main

build:
	rustc main.rs

bootstrap_server:
	./target/debug/test_server
