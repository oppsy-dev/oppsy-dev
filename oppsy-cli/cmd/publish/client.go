package publish

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

func publish(baseURL, workspaceID, name, tag string, packages []Package) (string, error) {
	body, err := json.Marshal(createManifestRequest(name, tag, packages))
	if err != nil {
		return "", err
	}

	url := strings.TrimRight(baseURL, "/") + "/v1/workspaces/" + workspaceID + "/manifests"
	resp, err := http.Post(url, "application/json; charset=utf-8", bytes.NewReader(body))
	if err != nil {
		return "", fmt.Errorf("post manifest: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("read response: %w", err)
	}

	if resp.StatusCode != http.StatusCreated {
		return "", fmt.Errorf("server returned %s: %s", resp.Status, strings.TrimSpace(string(respBody)))
	}

	var manifestID string
	if err := json.Unmarshal(respBody, &manifestID); err != nil {
		return "", fmt.Errorf("parse response: %w", err)
	}

	return manifestID, nil
}

type ManifestPackageJSON struct {
	Name      string `json:"name"`
	Version   string `json:"version"`
	Ecosystem string `json:"ecosystem"`
}

type CreateManifestRequestJSON struct {
	Name     string                `json:"name"`
	Tag      string                `json:"tag,omitempty"`
	Packages []ManifestPackageJSON `json:"packages"`
}

func createManifestRequest(name, tag string, packages []Package) CreateManifestRequestJSON {
	pkgs := make([]ManifestPackageJSON, len(packages))
	for i, p := range packages {
		pkgs[i] = ManifestPackageJSON(p)
	}
	return CreateManifestRequestJSON{Name: name, Tag: tag, Packages: pkgs}
}
