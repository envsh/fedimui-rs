package nulogic

/*
#include <stdint.h>

extern void runui();
extern void ffipxygocxrs(uintptr_t, uintptr_t);
extern void ffipxyrscxgoset(void*);
extern void ffipxyrscxgo();
*/
import "C"
import (
	"encoding/json"
	"log"
	"runtime"
	"time"
	"unsafe"

	"github.com/kitech/gopp"
)

//export ffipxyrscxgo
func ffipxyrscxgo(v unsafe.Pointer) {
	log.Println("hehehhee", v, time.Now())
}

var gortpin = runtime.Pinner{}

func init() {
	C.ffipxyrscxgoset(C.ffipxyrscxgo)
}

func ffipxygocxrswrap(v unsafe.Pointer) {
	var data = map[string]any{"cmd": "gosss测试1234$"}
	bcc, err := json.Marshal(data)
	gopp.ErrPrint(err)
	var s = string(bcc)
	var pp = uintptr(unsafe.Pointer(&s))
	// gortpin.Pin(pp)
	C.ffipxygocxrs(0, C.uintptr_t(pp))
}

func Main() {

	ffipxygocxrswrap(nil)

	// gopp.Forever()
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
