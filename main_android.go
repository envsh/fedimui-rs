package nulogic

import (
	mrand "math/rand"
	"unsafe"
)

/*
extern void android_mainrs(void*);
*/
import "C"

// https://developer.android.com/ndk/samples/sample_na?hl=zh-cn
// https://github.com/golang/go/wiki/GoArm
// https://www.cnblogs.com/kn-zheng/p/17004977.html

// tried Java_com_ncorti_kotlin_template_app_NativeLib_sayHello and Java_com_ncorti_kotlin_template_app_NativeLib_sayHello__

// export32bit Java_com_ncorti_kotlin_template_app_NativeLib_sayHello
//
//	func sayHello() int {
//		return mrand.Int()
//	}

//export Java_com_ncorti_kotlin_template_app_NativeLib_sayHello
func Java_com_ncorti_kotlin_template_app_NativeLib_sayHello() int {
	return mrand.Int()
}

//export android_main
func android_main(a unsafe.Pointer) {
	go nonuimain()

	C.android_mainrs(a) // loop forever here
}
