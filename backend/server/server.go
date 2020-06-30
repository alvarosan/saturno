package server

/**
 * C acessor for the array (it is possible to access directly in Go
 * with pointer arithmetic through unsafe.Pointer).
 *
 * https://stackoverflow.com/questions/49987098/how-to-access-a-c-pointer-array-from-golang
 */

// #cgo LDFLAGS: -L /home/alvaro/workspace/source/external/saturno/rendering/target/release -lrendering
//
// #include <stdlib.h>
//
// /**
//  * Reallocates renderer on every single frame.
//  */
// extern void* get_frame();
//
// /**
//  * Creates a renderer and returns a pointer to it. The allocated
//  * instance can be used to render frames (through render_scene).
//  */
// extern void* get_renderer(unsigned int scene);
// extern void* render_scene(void* renderer);
//
// /**
//  * Various helpers to get information from the frame or manipulate
//  * a scene.
//  */
// unsigned char get_value(void* frame, unsigned int x, unsigned int y,
//                         unsigned int c);
// extern unsigned int get_width(void* frame);
// extern unsigned int get_height(void* frame);
//
// /**
//  * Clean up.
//  */
// void drop_instance(void* instance) {
//      free(instance);
// }
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
	"strconv"
	"unsafe"
    "regexp"
)

var LISTENING_PORT = os.Getenv("LISTENING_PORT")
var SERVED_DIR = "dist"
var fserver = http.FileServer(http.Dir(SERVED_DIR))
var wasmFile = regexp.MustCompile("\\.wasm$")
var serverRenderer [10]unsafe.Pointer

func Initialize() {
	http.HandleFunc("/", customFileServer)
	http.HandleFunc("/api/v1/render", handleServerSideApp)

	fmt.Println("> Server initialized, listening on port " + LISTENING_PORT)
	log.Fatal(http.ListenAndServe(":"+LISTENING_PORT, nil))
}

func customFileServer(w http.ResponseWriter, r *http.Request) {
        rUri := r.RequestURI

        // Override *.wasm files
        if wasmFile.MatchString(rUri) {
            w.Header().Set("Content-Type", "application/wasm")
        }

        //http.StripPrefix("", fserver).ServeHTTP(w, r)
        fserver.ServeHTTP(w, r);
}

func handleServerSideApp(w http.ResponseWriter, r *http.Request) {
	fmt.Println("> Serving serverside-rendering app ... ")

	sceneId, err := strconv.ParseUint(r.URL.Query().Get("sceneId"), 10, 64)
	if err != nil {
		log.Println(">> Url Param 'sceneId' is missing !")
		http.NotFound(w, r)
		return
	}

	frame := getFrame(sceneId)
	buffer := new(bytes.Buffer)
	err = png.Encode(buffer, frame)
	checkErr(err)

	w.Header().Set("Content-Type", "image/png")
	w.Header().Set("Content-Length", strconv.Itoa(len(buffer.Bytes())))
	_, err = w.Write(buffer.Bytes())
	checkErr(err)
}

func getFrame(sceneId uint64) image.Image {
        if serverRenderer[sceneId] == nil {
        	serverRenderer[sceneId] =
            unsafe.Pointer(C.get_renderer(C.uint(sceneId)))
        }
	var framePtr = unsafe.Pointer(C.render_scene(serverRenderer[sceneId]))

    // Or print the frame directly (reallocates renderer every time)
	//var framePtr = unsafe.Pointer(C.get_frame())

    // TODO Avoid this brute force copy of the frame (create a C interface for
    // slices?)
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

	C.drop_instance(framePtr)
	return goFrame
}

func checkErr(err error) {
	if err != nil {
		fmt.Println(">! Panicked over: ", err.Error())
		panic(err)
	}
}

func CleanUp() {
    for i := 0; i < 10; i++ {
        if serverRenderer[i] != nil {
            C.drop_instance(serverRenderer[i])
        }
    }
}
