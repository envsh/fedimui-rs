NDKDIR=/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK/toolchains/llvm/prebuilt/darwin-x86_64

all: rs go

rs:
	cargo build

go:
	go build
	cd app && go build -o ../mainapp

####### depcreated
goso:
	cd app && go build -o ../mainapp.so -buildmode=c-shared

####### see app/buildandroid.sh
andso:
	cd app && sh ./buildandroid.sh
	# cd app && GOOS=android GOARCH=arm64 CGO_ENABLED=1 go build -v -x -buildmode=c-shared
	#GOOS=android GOARCH=arm64 GO build -x main.go
	# cd app && go build -target android -arch arm64 -o ../mainapp.so -buildmode=c-shared

rsdland:
	rustup target add armv7-linux-androideabi

rsbdand:
	cargo ndk -t armeabi-v7a build
	# otool -L target/armv7-linux-androideabi/debug/libfedimuirs.so

andapk:
	mkdir -p jniLibs
	cp target/armv7-linux-androideabi/debug/libfedimuirs.so jniLibs/
	cp fedimuigo.so jniLibs/libfedimuigo.so
	
	$(NDKDIR)/bin/llvm-strip -S jniLibs/libfedimuirs.so
	$(NDKDIR)/bin/llvm-strip -S jniLibs/libfedimuigo.so
	ls -lh jniLibs/
	$(NDKDIR)/bin/llvm-objdump -x jniLibs/libfedimuirs.so |grep NEEDED
