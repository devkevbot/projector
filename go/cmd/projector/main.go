package main

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/devkevbot/projector/pkg/cli"
)

func main() {
	opts, err := cli.GetOpts()
	if err != nil {
		log.Fatalf("unable to get options %v", err)
	}

	config, err := cli.NewConfig(opts)
	if err != nil {
		log.Fatalf("unable to parse config %v", err)
	}

	proj := cli.NewProjector(config)

	if config.Operation == cli.Print {
		if len(config.Args) == 0 {
			data := proj.GetValueAll()
			jsonString, err := json.Marshal(data)
			if err != nil {
				log.Fatalf("unable to JSON-ify projector %v", err)
			}
			fmt.Printf("%v", string(jsonString))
		} else if data, ok := proj.GetValue(config.Args[0]); ok {
			fmt.Printf("%v", data)
		}
	}

	if config.Operation == cli.Add {
		proj.SetValue(config.Args[0], config.Args[1])
		err := proj.Save()
		if err != nil {
			log.Fatalf("failed to save %v", err)
		}
	}

	if config.Operation == cli.Remove {
		proj.RemoveValue(config.Args[0])
		err := proj.Save()
		if err != nil {
			log.Fatalf("failed to save %v", err)
		}
	}
}
