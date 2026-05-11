package publish

import (
	"context"
	"io"
	"log/slog"

	"github.com/urfave/cli/v3"
)

func Command(_, _ io.Writer) *cli.Command {
	return &cli.Command{
		Name:        "publish",
		Usage:       "Parse a manifest file and publish its packages to an Oppsy workspace",
		Description: "Reads a dependency manifest file (e.g. Cargo.lock, package-lock.json) from the given path, extracts all packages, and publishes them to the specified Oppsy workspace under the given name and tag.",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:     "host-url",
				Aliases:  []string{"H"},
				Required: true,
				Usage:    "Base URL of the Oppsy service to publish to (e.g. https://oppsy.example.com)",
			},
			&cli.StringFlag{
				Name:     "workspace-id",
				Aliases:  []string{"W"},
				Required: true,
				Usage:    "ID of the workspace that will own this manifest",
			},
			&cli.StringFlag{
				Name:      "lockfile",
				Aliases:   []string{"L"},
				Required:  true,
				TakesFile: true,
				Usage:     "Path to the manifest file on disk (e.g. ./Cargo.lock, ./package-lock.json)",
			},
			&cli.StringFlag{
				Name:     "name",
				Aliases:  []string{"N"},
				Required: true,
				Usage:    "Logical name for the manifest within the workspace (e.g. my-service)",
			},
			&cli.StringFlag{
				Name:    "tag",
				Aliases: []string{"T"},
				Usage:   "Version label for this publish — used to track changes over time (e.g. a git SHA or semver string)",
			},
		},
		Action: func(_ context.Context, cmd *cli.Command) error {
			return runPublish(cmd)
		},
	}
}

func runPublish(cmd *cli.Command) error {
	packages, err := scan(cmd.String("lockfile"))
	if err != nil {
		return err
	}

	manifestID, err := publish(cmd.String("host-url"), cmd.String("workspace-id"), cmd.String("name"), cmd.String("tag"), packages)
	if err != nil {
		return err
	}

	slog.Info("published manifest", "id", manifestID)
	return nil
}
