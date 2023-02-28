package main

import (
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

	fmt.Printf("opts: %+v", opts)
	fmt.Printf("config: %+v", config)
}
