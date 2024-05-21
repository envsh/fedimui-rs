
export CGO_CFLAGS="-I/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK//toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include -I/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK//toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/arm-linux-androideabi"

export CC=/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK//toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang

# export CC=/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK//toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang

# export LD=/usr/local/Caskroom/android-ndk/26d/AndroidNDK11579264.app/Contents/NDK//toolchains/llvm/prebuilt/darwin-x86_64/bin/ld

GOOS=android  GOARCH=arm  CGO_LDFLAGS="-arch arm" CGO_ENABLED=1 go build -v -x -buildmode=c-shared -o ../fedimui.rs.so
# GOOS=android GOARCH=arm64 CGO_ENABLED=1 go build -v -x -buildmode=c-shared
