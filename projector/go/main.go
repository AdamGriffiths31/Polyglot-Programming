package main

import (
	"adam/pkg/config"
	"adam/pkg/projector"
	"fmt"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v\n", opts)

	projector, err := projector.NewConfig(opts)
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v\n", projector)
}
