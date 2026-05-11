package cmd

import (
	"context"
	"io"
	"log/slog"

	"github.com/urfave/cli/v3"
)

type CommandBuilder = func(stdout, stderr io.Writer) *cli.Command

func Run(args []string, stdout, stderr io.Writer, commands []CommandBuilder) int {
	cmds := make([]*cli.Command, 0, len(commands))
	for _, cmd := range commands {
		cmds = append(cmds, cmd(stdout, stderr))
	}

	app := &cli.Command{
		Name:           "oppsy-cli",
		Usage:          "A CLI helper for interacting with the Oppsy API — publish manifests, manage workspaces, and more",
		Suggest:        true,
		Writer:         stdout,
		ErrWriter:      stderr,
		DefaultCommand: "publish",
		Commands:       cmds,
	}

	err := app.Run(context.Background(), args)
	if err != nil {
		slog.Error(err.Error())
		return 1
	}
	return 0
}
