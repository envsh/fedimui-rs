all:
	cargo build
	cd app && go build -o ../mainapp
