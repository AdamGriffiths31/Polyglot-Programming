package main

import (
	"adam/pkg/config"
	"adam/pkg/projector"
	"encoding/json"
	"fmt"
	"log"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		panic(err)
	}

	config, err := projector.NewConfig(opts)
	if err != nil {
		panic(err)
	}

	proj := projector.NewProjector(config)

	if config.Operation == projector.Print {
		if len(config.Args) == 0 {
			data := proj.GetValueAll()
			jsonString, err := json.Marshal(data)
			if err != nil {
				log.Fatal(err)
			}
			fmt.Printf("%v", string(jsonString))
		} else {
			value, ok := proj.GetValue(config.Args[0])
			if ok {
				fmt.Printf("%v", value)
			}
		}
	}

	if config.Operation == projector.Add {
		proj.SetValue(config.Args[0], config.Args[1])
		proj.Save()
	}

	if config.Operation == projector.Remove {
		proj.RemoveValue(config.Args[0])
		proj.Save()
	}
}
