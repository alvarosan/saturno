package server

import (
        "testing"
        "os"
        "image"
        "image/png"
)

func TestGetFrame(t *testing.T) {
	frame := GetFrame()
	writeImageToFile(frame)

//	got := Abs(-1)
//	if got != 1 {
//		t.Errorf("Abs(-1) = %d; want 1", got)
//	}
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

