all: rs go

rs:
	cargo build

go:
	go build
	cd app && go build -o ../mainapp

goso:
	cd app && go build -o ../mainapp.so -buildmode=c-shared

andso:
	cd app && go build -target android -arch arm64 -o ../mainapp.so -buildmode=c-shared