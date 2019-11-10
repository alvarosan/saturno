package server

/**
 * C acessor for the array (it is possible to access directly in Go
 * with pointer arithmetic through unsafe.Pointer).
 *
 * https://stackoverflow.com/questions/49987098/how-to-access-a-c-pointer-array-from-golang
 */

// #cgo LDFLAGS: -L /home/alvaro/workspace/source/saturno/rendering/target/release  -lrendering_c_abi
//
// extern unsigned char* get_frame();
//
// unsigned char get_value(void* data, int index) {
//     unsigned char* data_uchar = (unsigned char*) data;
//     return data_uchar[index];
// }
import "C"

import (
    "fmt"
    "image"
    "image/png"
    "image/color"
    "log"
    "net/http"
    "os"
    "unsafe"
)

var LISTENING_PORT = os.Getenv("LISTENING_PORT")

func Initialize() {
    http.HandleFunc("/clientside", handleClientSideApp)
    http.HandleFunc("/serverside", handleServerSideApp)
    fmt.Println("> Server initialized, listening on port " + LISTENING_PORT)

    log.Fatal(http.ListenAndServe(":"+LISTENING_PORT, nil))
}

func handleClientSideApp(w http.ResponseWriter, r *http.Request) {
    fmt.Println("> Serving clientside-rendering app ... ")
    fmt.Fprintf(w, "PONG! /clientside API '%v'", r.Method)
}

func handleServerSideApp(w http.ResponseWriter, r *http.Request) {
    fmt.Println("> Serving serverside-rendering app ... ")
    //fmt.Fprintf(w, "PONG! /serverside API '%v' '%d'", r.Method, result)
}

func WrapImage() {
    var height int = 100
    var width int = 200
    myImage := image.NewNRGBA(image.Rect(0, 0, width, height))
    var frame_ptr = unsafe.Pointer(C.get_frame())

    for y := 0; y < height; y++ {
        for x := 0; x < width; x++ {
            red := uint8(C.get_value(frame_ptr, C.int(3 * ( y * width + x ))))
            green := uint8(C.get_value(frame_ptr, C.int(3 * ( y * width + x ) + 1)))
            blue := uint8(C.get_value(frame_ptr, C.int(3 * ( y * width + x ) + 2)))
            alpha := uint8(C.get_value(frame_ptr, C.int(3 * ( y * width + x ) + 3)))
            myImage.SetNRGBA(x, y, color.NRGBA{red, green, blue, alpha})
        }
    }

    writeImageToFile(myImage)
}

func writeImageToFile(img image.Image) {
    // outputFile is a File type which satisfies Writer interface
    outputFile, err := os.Create("my_test.png")
    checkErr(err)

    // Encode takes a writer interface and an image interface
    // We pass it the File and the RGBA
    png.Encode(outputFile, img)

    // Don't forget to close files
    outputFile.Close()
}

func checkErr(err error) {
    if err != nil {
        fmt.Println(">! Panicked over: ", err.Error())
        panic(err)
    }
}
