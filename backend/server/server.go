package server

// #cgo LDFLAGS: -L /home/alvaro/workspace/source/saturno/rendering/target/release  -lrendering_c_abi
// extern unsigned char* get_frame();
import "C"

import (
	"fmt"
	"image"
	"image/png"
	"log"
	"net/http"
	"os"
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
	//var result = C.get_frame()

	myImage := image.NewNRGBA(image.Rect(0, 0, 200, 100))

	// outputFile is a File type which satisfies Writer interface
	outputFile, err := os.Create("my_test.png")
        checkErr(err)

	// Encode takes a writer interface and an image interface
	// We pass it the File and the RGBA
	png.Encode(outputFile, myImage)

	// Don't forget to close files
	outputFile.Close()
}

func checkErr(err error) {
	if err != nil {
		fmt.Println(">! Panicked over: ", err.Error())
		panic(err)
	}
}
