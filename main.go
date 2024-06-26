package nulogic

import (
	"encoding/json"
	"fmt"
	"log"
	"runtime"
	"time"
	"unsafe"

	"github.com/kitech/gopp"
)

//go:generate goppgen nulogic cgo
// what

/*
#include <stdint.h>

extern void runui();
extern void ffipxygocxrs(uintptr_t, uintptr_t);
extern void ffipxygocxrs_viajson(uintptr_t, uintptr_t);
extern void ffipxygocxrs_viastruct(uintptr_t, uintptr_t);
extern void ffipxyrscxgoset(void*);
extern void ffipxyrscxgo();
*/
import "C"

//export ffipxyrscxgo
func ffipxyrscxgo(v unsafe.Pointer) {
	log.Println("hehehhee", v, time.Now())
}

var gortpin = runtime.Pinner{}

func init() {
	C.ffipxyrscxgoset(C.ffipxyrscxgo)
}

func ffipxygocxrswrap(v vptr) {
	var data = map[string]any{"cmd": "gosss测试1234$"}
	bcc, err := json.Marshal(data)
	gopp.ErrPrint(err)
	var s = string(bcc)
	// var pp = uintptr(unsafe.Pointer(&s))
	// gortpin.Pin(pp)
	C.ffipxygocxrs(0, anyptr2uptrc(&s))
}

// todo
func ffipxygocxrs_viajson_wrap(v vptr) {
	var data = map[string]any{"cmd": "gosss测试1234$"}
	bcc, err := json.Marshal(data)
	gopp.ErrPrint(err)
	var s = string(bcc)
	// var pp = uintptr(unsafe.Pointer(&s))
	// gortpin.Pin(pp)
	C.ffipxygocxrs(0, anyptr2uptrc(&s))
}

type ffipassbyst struct {
	cmd  *string
	argc usize
	arg0 *string
	arg1 *string
	arg2 *string
	arg3 *string
	arg4 *string
	arg5 *string
}

// todo
func ffipxygocxrs_viastruct_wrap(v vptr) {
	var data = map[string]any{"cmd": "gosss测试1234$"}
	bcc, err := json.Marshal(data)
	gopp.ErrPrint(err)
	var s = string(bcc)
	// var pp = uintptr(unsafe.Pointer(&s))
	// gortpin.Pin(pp)
	C.ffipxygocxrs(0, anyptr2uptrc(&s))
}

func nonuimain() {
	type callpxyst struct {
		Cmd  string   `json:"cmd"`
		Args []string `json:"args"`
	}

	for i := 0; i < 160; i++ {
		if true {
			// break
		}
		// gopp.SleepSec(3)
		gopp.SleepMs(100)
		// time.Sleep(300*time.Millisecond)
		obj := callpxyst{Cmd: "newmessage", Args: []string{fmt.Sprintf("msgfromgooo%d", i), "a1", "a2"}}
		scc := gopp.JsonMarshalMust(obj)
		log.Println(scc)
		// var pp = uintptr(unsafe.Pointer(&scc))
		// C.ffipxygocxrs(0, C.uintptr_t(pp))
		C.ffipxygocxrs(0, anyptr2uptrc(&scc))
	}
}

func Main() {

	// if r := recover(); r != nil {
	// 	println("hhhhh", r)
	// }

	ffipxygocxrswrap(nil)

	go nonuimain()
	// gopp.Forever()
	C.runui() // forever loop
}

/*
在android上listwidget滚动也是CPU 100%，360条的时候
剩下的唯一一个优势就是占用内存小，是几个跨平台中最小的
*/

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
