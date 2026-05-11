package publish

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"path/filepath"

	scalibr "github.com/google/osv-scalibr"
	scalibrfs "github.com/google/osv-scalibr/fs"
	pl "github.com/google/osv-scalibr/plugin/list"
	"github.com/urfave/cli/v3"
)

func Command(stdout, _ io.Writer) *cli.Command {
	return &cli.Command{
		Name:        "publish",
		Usage:       "Parse a manifest file and publish its packages to an Oppsy workspace",
		Description: "Reads a dependency manifest file (e.g. Cargo.lock, package-lock.json) from the given path, extracts all packages, and publishes them to the specified Oppsy workspace under the given name and tag.",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    "host-url",
				Aliases: []string{"H"},
				// Required: true,
				Usage: "Base URL of the Oppsy service to publish to (e.g. https://oppsy.example.com)",
			},
			&cli.StringFlag{
				Name:    "workspace-id",
				Aliases: []string{"W"},
				// Required: true,
				Usage: "ID of the workspace that will own this manifest",
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
		Action: runPublish,
	}
}

type ManifestPackage struct {
	Name      string `json:"name"`
	Version   string `json:"version"`
	Ecosystem string `json:"ecosystem"`
}

type CreateManifestRequest struct {
	Name     string            `json:"name"`
	Tag      string            `json:"tag,omitempty"`
	Packages []ManifestPackage `json:"packages"`
}

func runPublish(_ context.Context, cmd *cli.Command) error {
	packages, err := scan(cmd.String("lockfile"))
	if err != nil {
		return err
	}

	req := CreateManifestRequest{
		Name:     cmd.String("name"),
		Tag:      cmd.String("tag"),
		Packages: packages,
	}

	b, err := json.Marshal(req)
	if err != nil {
		return err
	}
	println(string(b))
	return nil
}

func scan(lockfilePath string) ([]ManifestPackage, error) {
	rootDir := filepath.Dir(lockfilePath)

	plugins, err := pl.FromNames([]string{"sourcecode"}, nil)
	if err != nil {
		return nil, fmt.Errorf("load extractors: %w", err)
	}

	cfg := &scalibr.ScanConfig{
		Plugins:        plugins,
		ScanRoots:      []*scalibrfs.ScanRoot{scalibrfs.RealFSScanRoot(rootDir)},
		PathsToExtract: []string{lockfilePath},
		IgnoreSubDirs:  true,
	}

	result := scalibr.New().Scan(context.Background(), cfg)
	if result == nil {
		return nil, errors.New("osv-scalibr scan returned nil result")
	}

	out := make([]ManifestPackage, 0, len(result.Inventory.Packages))
	for _, p := range result.Inventory.Packages {
		if p == nil || p.Name == "" {
			continue
		}
		out = append(out, ManifestPackage{
			Name:      p.Name,
			Version:   p.Version,
			Ecosystem: p.Ecosystem().String(),
		})
	}

	return out, nil
}
