package main

import (
	"C"
	"io"
	"os"
)

//export ReadTextFileSize
func ReadTextFileSize(path *C.char) C.int {
	p := C.GoString(path)
	r, err := os.Open(p)
	if err != nil {
		panic(err)
	}

	defer r.Close()
	b, err := io.ReadAll(r)
	if err != nil {
		panic(err)
	}

	return C.int(len(b))
}

func main() {}
