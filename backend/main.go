package main

import "./server"

func main() {
        server.Initialize()
        server.CleanUp()

	// TODO Review this way of linking Rust into Go
	// https://blog.filippo.io/rustgo/
}
