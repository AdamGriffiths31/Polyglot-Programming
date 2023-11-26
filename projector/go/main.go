package main

import (
	"adam/pkg/config"
	"fmt"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		panic(err)
	}

	fmt.Printf("opts: %v\n", opts)
}
