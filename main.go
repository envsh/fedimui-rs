package main

/*


#cgo CFLAGS: -I${SRCDIR}/target/debug
#cgo LDFLAGS: -Wl,-rpath,${SRCDIR}/target/debug
#cgo LDFLAGS: -L${SRCDIR}/target/debug
#cgo LDFLAGS: -lmy_project
// #cgo LDFLAGS: -l

// #include <test.h>

extern void runui();
*/
import "C"

func main() {
	C.runui()
}

/*
docssssssss

Whatever:my-project $ otool  -L target/debug/libmy_project.dylib | awk '{print $1}'
target/debug/libmy_project.dylib:
/Users//aprog/my-project/target/debug/deps/libmy_project.dylib
/System/Library/Frameworks/AppKit.framework/Versions/C/AppKit
/System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
/System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
/System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
/System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
/usr/lib/libSystem.B.dylib
/usr/lib/libobjc.A.dylib
/System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
/System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
/System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
/System/Library/Frameworks/Carbon.framework/Versions/A/Carbon
/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
/usr/lib/libiconv.2.dylib
/System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
/System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices

*/
