package main

import (
	"os"

	"github.com/oppsy-dev/oppsy/oppsy-cli/cmd"
	"github.com/oppsy-dev/oppsy/oppsy-cli/cmd/publish"
)

func main() {
	os.Exit(
		cmd.Run(os.Args, os.Stdout, os.Stderr, []cmd.CommandBuilder{
			publish.Command,
		}),
	)
}
