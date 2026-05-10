package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	scalibr "github.com/google/osv-scalibr"
	scalibrfs "github.com/google/osv-scalibr/fs"
	pl "github.com/google/osv-scalibr/plugin/list"
)

type manifestPackage struct {
	Name      string `json:"name"`
	Version   string `json:"version"`
	Ecosystem string `json:"ecosystem"`
}

type createManifestRequest struct {
	Name     string            `json:"name"`
	Tag      string            `json:"tag,omitempty"`
	Packages []manifestPackage `json:"packages"`
}

func main() {
	if len(os.Args) < 2 {
		usage()
		os.Exit(2)
	}

	switch os.Args[1] {
	case "publish":
		if len(os.Args) != 7 {
			usage()
			os.Exit(2)
		}
		if err := runPublish(os.Args[2], os.Args[3], os.Args[4], os.Args[5], os.Args[6]); err != nil {
			fmt.Fprintln(os.Stderr, "error:", err)
			os.Exit(1)
		}
	case "-h", "--help", "help":
		usage()
	default:
		fmt.Fprintf(os.Stderr, "unknown command %q\n", os.Args[1])
		usage()
		os.Exit(2)
	}
}

func usage() {
	fmt.Fprintln(os.Stderr, "usage: oppsy-cli publish <oppsy-service-url> <workspace-id> <path-to-the-manifest-file> <name> <tag>")
}

func runPublish(serviceURL, workspaceID, manifestPath, name, tag string) error {
	pkgs, err := extractPackages(manifestPath)
	if err != nil {
		return fmt.Errorf("extract packages: %w", err)
	}
	if len(pkgs) == 0 {
		return errors.New("no packages extracted from manifest")
	}

	body := createManifestRequest{
		Name:     name,
		Tag:      tag,
		Packages: pkgs,
	}
	buf, err := json.MarshalIndent(body, "", "  ")
	if err != nil {
		return fmt.Errorf("encode request: %w", err)
	}

	endpoint := fmt.Sprintf("%s/v1/workspaces/%s/manifests",
		strings.TrimRight(serviceURL, "/"), workspaceID)

	fmt.Fprintf(os.Stderr, "POST %s\n", endpoint)
	fmt.Fprintln(os.Stderr, string(buf))

	dumpPath := "oppsy-cli-payload.json"
	if err := os.WriteFile(dumpPath, buf, 0o644); err != nil {
		return fmt.Errorf("write payload dump: %w", err)
	}
	fmt.Fprintf(os.Stderr, "payload saved to %s\n", dumpPath)

	req, err := http.NewRequestWithContext(context.Background(),
		http.MethodPost, endpoint, bytes.NewReader(buf))
	if err != nil {
		return fmt.Errorf("build request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json; charset=utf-8")
	req.Header.Set("Accept", "application/json; charset=utf-8")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return fmt.Errorf("send request: %w", err)
	}
	defer resp.Body.Close()

	rb, _ := io.ReadAll(resp.Body)
	if resp.StatusCode/100 != 2 {
		return fmt.Errorf("server returned %d: %s", resp.StatusCode, strings.TrimSpace(string(rb)))
	}

	fmt.Printf("published %d package(s); manifest_id=%s\n", len(pkgs), strings.Trim(string(rb), "\"\n "))
	return nil
}

func extractPackages(path string) ([]manifestPackage, error) {
	abs, err := filepath.Abs(path)
	if err != nil {
		return nil, err
	}
	info, err := os.Stat(abs)
	if err != nil {
		return nil, err
	}
	if info.IsDir() {
		return nil, fmt.Errorf("%s: expected file, got directory", path)
	}

	rootDir := filepath.Dir(abs)

	plugins, err := pl.FromNames([]string{"sourcecode"}, nil)
	if err != nil {
		return nil, fmt.Errorf("load extractors: %w", err)
	}

	cfg := &scalibr.ScanConfig{
		Plugins:        plugins,
		ScanRoots:      []*scalibrfs.ScanRoot{scalibrfs.RealFSScanRoot(rootDir)},
		PathsToExtract: []string{abs},
		IgnoreSubDirs:  true,
	}

	result := scalibr.New().Scan(context.Background(), cfg)
	if result == nil {
		return nil, errors.New("scan returned nil result")
	}

	out := make([]manifestPackage, 0, len(result.Inventory.Packages))
	for _, p := range result.Inventory.Packages {
		if p == nil || p.Name == "" {
			continue
		}
		eco := p.Ecosystem().String()
		out = append(out, manifestPackage{
			Name:      p.Name,
			Version:   p.Version,
			Ecosystem: eco,
		})
	}
	return out, nil
}
