package publish

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"path/filepath"

	scalibr "github.com/google/osv-scalibr"
	scalibrfs "github.com/google/osv-scalibr/fs"
	scalibrlog "github.com/google/osv-scalibr/log"
	pl "github.com/google/osv-scalibr/plugin/list"
)

type Package struct {
	Name      string
	Version   string
	Ecosystem string
}

func scan(lockfilePath string) ([]Package, error) {
	scalibrlog.SetLogger(&noOpLogger{})

	plugins, err := pl.FromNames([]string{"sourcecode"}, nil)
	if err != nil {
		return nil, fmt.Errorf("load extractors: %w", err)
	}

	scanRoot, err := filepath.Abs(filepath.Dir(lockfilePath))
	if err != nil {
		return nil, fmt.Errorf("resolve scan root: %w", err)
	}

	cfg := &scalibr.ScanConfig{
		Plugins:        plugins,
		ScanRoots:      []*scalibrfs.ScanRoot{scalibrfs.RealFSScanRoot(scanRoot)},
		PathsToExtract: []string{lockfilePath},
		IgnoreSubDirs:  true,
	}

	result := scalibr.New().Scan(context.Background(), cfg)
	if result == nil {
		return nil, errors.New("osv-scalibr scan returned nil result")
	}

	scannedPaths := make(map[string]struct{})
	packages := make([]Package, 0, len(result.Inventory.Packages))
	for _, p := range result.Inventory.Packages {
		if p == nil || p.Name == "" {
			continue
		}
		for _, loc := range p.Locations {
			scannedPaths[filepath.Join(scanRoot, loc)] = struct{}{}
		}
		packages = append(packages, Package{
			Name:      p.Name,
			Version:   p.Version,
			Ecosystem: p.Ecosystem().String(),
		})
	}

	for path := range scannedPaths {
		slog.Info("scanned lockfile", "path", path)
	}
	slog.Info("scan complete", "packages", len(packages))

	return packages, nil
}

type noOpLogger struct{}

func (noOpLogger) Errorf(string, ...any) {}
func (noOpLogger) Warnf(string, ...any)  {}
func (noOpLogger) Infof(string, ...any)  {}
func (noOpLogger) Debugf(string, ...any) {}
func (noOpLogger) Error(...any)          {}
func (noOpLogger) Warn(...any)           {}
func (noOpLogger) Info(...any)           {}
func (noOpLogger) Debug(...any)          {}
