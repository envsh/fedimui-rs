NDKDIR=/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK/toolchains/llvm/prebuilt/darwin-x86_64

all: rs go

rs:
	cargo build
# cargo build --release

go:
	go build
	cd appgo && go build -o ../mainapp

####### depcreated
goso:
	cd appgo && go build -o ../mainapp.so -buildmode=c-shared

####### see app/buildandroid.sh
andgo:
	cd appgo && sh ./buildandroid.sh
	$(NDKDIR)/bin/llvm-objdump -x ./fedimuigo.so |grep NEEDED
	# cd app && GOOS=android GOARCH=arm64 CGO_ENABLED=1 go build -v -x -buildmode=c-shared
	#GOOS=android GOARCH=arm64 GO build -x main.go
	# cd app && go build -target android -arch arm64 -o ../mainapp.so -buildmode=c-shared

rsdland:
	rustup target add armv7-linux-androideabi
	# alias of arm64-v8a
	rustup target add aarch64-linux-android

andrs:
	#cargo ndk -t armeabi-v7a build
	cargo ndk -t aarch64-linux-android build
	# otool -L target/armv7-linux-androideabi/debug/libfedimuirs.so

andapk:
	mkdir -p jniLibs
	# cp target/armv7-linux-androideabi/debug/libfedimuirs.so jniLibs/
	cp target/aarch64-linux-android/debug/libfedimuirs.so jniLibs/
	cp fedimuigo.so jniLibs/libfedimuigo.so
	
	$(NDKDIR)/bin/llvm-strip -S jniLibs/libfedimuirs.so
	$(NDKDIR)/bin/llvm-strip -S jniLibs/libfedimuigo.so
	ls -lh jniLibs/
	$(NDKDIR)/bin/llvm-objdump -x jniLibs/libfedimuirs.so |grep NEEDED
	$(NDKDIR)/bin/llvm-objdump -x jniLibs/libfedimuigo.so |grep NEEDED
	$(NDKDIR)/bin/llvm-objdump -x jniLibs/libfedimuigo.so |grep say

	# cp jniLibs/lib*.so kotlin-android-template/app/src/main/jniLibs/armeabi-v7a/
	cp jniLibs/lib*.so kotlin-android-template/app/src/main/jniLibs/arm64-v8a/

	cd kotlin-android-template/ && gradle build
	ls -lh kotlin-android-template/app/build/outputs/apk/debug/

andall: andrs andgo andapk

andapk2:
	# for rust cargo apk 
	# https://releases.slint.dev/1.6.0/docs/rust/slint/android/
	# The ANDROID_HOME and ANDROID_NDK_ROOT environment
	cargo apk run  --target aarch64-linux-android --lib