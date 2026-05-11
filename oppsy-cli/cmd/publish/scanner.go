package publish

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"

	scalibr "github.com/google/osv-scalibr"
	scalibrfs "github.com/google/osv-scalibr/fs"
	pl "github.com/google/osv-scalibr/plugin/list"
)

type Package struct {
	Name      string
	Version   string
	Ecosystem string
}

func scan(lockfilePath string) ([]Package, error) {
	plugins, err := pl.FromNames([]string{"sourcecode"}, nil)
	if err != nil {
		return nil, fmt.Errorf("load extractors: %w", err)
	}

	cfg := &scalibr.ScanConfig{
		Plugins:        plugins,
		ScanRoots:      []*scalibrfs.ScanRoot{scalibrfs.RealFSScanRoot(filepath.Dir(lockfilePath))},
		PathsToExtract: []string{lockfilePath},
		IgnoreSubDirs:  true,
	}

	result := scalibr.New().Scan(context.Background(), cfg)
	if result == nil {
		return nil, errors.New("osv-scalibr scan returned nil result")
	}

	packages := make([]Package, 0, len(result.Inventory.Packages))
	for _, p := range result.Inventory.Packages {
		if p == nil || p.Name == "" {
			continue
		}
		packages = append(packages, Package{
			Name:      p.Name,
			Version:   p.Version,
			Ecosystem: p.Ecosystem().String(),
		})
	}

	return packages, nil
}
