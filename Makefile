all: rs go

rs:
	cargo build

go:
	go build
	cd app && go build -o ../mainapp

goso:
	cd app && go build -o ../mainapp.so -buildmode=c-shared

andso:
	cd app && sh ./buildandroid.sh
	# cd app && GOOS=android GOARCH=arm64 CGO_ENABLED=1 go build -v -x -buildmode=c-shared
	#GOOS=android GOARCH=arm64 GO build -x main.go
	# cd app && go build -target android -arch arm64 -o ../mainapp.so -buildmode=c-shared