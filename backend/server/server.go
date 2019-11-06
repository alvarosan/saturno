package server

// #cgo LDFLAGS: -L /home/alvaro/workspace/source/saturno/rendering/target/release  -lrendering_c_abi
// extern void* get_frame();
import "C"

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

var LISTENING_PORT = os.Getenv("LISTENING_PORT")

func Initialize() {
	http.HandleFunc("/clientside", handleClientSideApp)
	http.HandleFunc("/serverside", handleServerSideApp)
	fmt.Println("> Server initialized, listening on port " + LISTENING_PORT)

	log.Fatal(http.ListenAndServe(":" + LISTENING_PORT, nil))
}

func handleClientSideApp(w http.ResponseWriter, r *http.Request) {
	fmt.Println("> Serving clientside-rendering app ... ")
	fmt.Fprintf(w, "PONG! /clientside API '%v'", r.Method)
}

func handleServerSideApp(w http.ResponseWriter, r *http.Request) {
	fmt.Println("> Serving serverside-rendering app ... ")

        var result = C.get_frame();
	fmt.Fprintf(w, "PONG! /serverside API '%v' '%d'", r.Method, result)

}

func checkErr(err error) {
	if err != nil {
		fmt.Println(">! Panicked over: ", err.Error())
		panic(err)
	}
}
