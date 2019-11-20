package server

/**
 * C acessor for the array (it is possible to access directly in Go
 * with pointer arithmetic through unsafe.Pointer).
 *
 * https://stackoverflow.com/questions/49987098/how-to-access-a-c-pointer-array-from-golang
 */

// #cgo LDFLAGS: -L /home/alvaro/workspace/source/saturno/rendering/target/release -lrendering
//
// #include <stdlib.h>
//
// extern void* get_frame();
// unsigned char get_value(void* frame, unsigned int x, unsigned int y, unsigned int c);
// extern unsigned int get_width(void* frame);
// extern unsigned int get_height(void* frame);
//
// void drop_frame(void* frame) {
//      free(frame);
// }
//
import "C"

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"log"
	"net/http"
	"os"
	//"time"
	"strconv"
	"unsafe"
)

var LISTENING_PORT = os.Getenv("LISTENING_PORT")

func Initialize() {

	fs := http.FileServer(http.Dir("dist"))
	http.Handle("/", http.StripPrefix("", fs))

	http.HandleFunc("/api/v1/render", handleServerSideApp)
	fmt.Println("> Server initialized, listening on port " + LISTENING_PORT)

	log.Fatal(http.ListenAndServe(":"+LISTENING_PORT, nil))
}

func handleServerSideApp(w http.ResponseWriter, r *http.Request) {
	fmt.Println("> Serving serverside-rendering app ... ")

	frame := GetFrame()
	buffer := new(bytes.Buffer)
	err := png.Encode(buffer, frame)
	checkErr(err)

	w.Header().Set("Content-Type", "image/png")
	w.Header().Set("Content-Length", strconv.Itoa(len(buffer.Bytes())))
	_, err = w.Write(buffer.Bytes())
	checkErr(err)
}

func GetFrame() image.Image {
	var framePtr = unsafe.Pointer(C.get_frame())
	var width int = int(C.get_width(framePtr))
	var height int = int(C.get_height(framePtr))
	goFrame := image.NewNRGBA(image.Rect(0, 0, width, height))

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {

			r := uint8(C.get_value(framePtr, C.uint(x), C.uint(y), 0))
			g := uint8(C.get_value(framePtr, C.uint(x), C.uint(y), 1))
			b := uint8(C.get_value(framePtr, C.uint(x), C.uint(y), 2))
			a := uint8(C.get_value(framePtr, C.uint(x), C.uint(y), 3))
			goFrame.SetNRGBA(x, y, color.NRGBA{r, g, b, a})
		}
	}

	C.drop_frame(framePtr)
	return goFrame
}

func checkErr(err error) {
	if err != nil {
		fmt.Println(">! Panicked over: ", err.Error())
		panic(err)
	}
}
