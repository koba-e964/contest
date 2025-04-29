package main

// TODO: respect depends_on

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/pelletier/go-toml/v2"
)

type Entry struct {
	Prefix    string   `json:"prefix"`
	Filename  string   `json:"filename"`
	DependsOn []string `json:"depends_on,omitempty"`
}

type Config struct {
	Location string `toml:"location"`
	Rust     struct {
		Entries []Entry `toml:"entries"`
	} `toml:"rust"`
	Cpp struct {
		Entries []Entry `toml:"entries"`
	} `toml:"cpp"`
}

type SnippetEntry struct {
	Scope       string   `json:"scope"`
	Prefix      string   `json:"prefix"`
	Body        []string `json:"body"`
	Description string   `json:"description"`
}

func main() {
	// Read the vscode_config.toml file
	configPath := "vscode_config.toml"
	configData, err := os.ReadFile(configPath)
	if err != nil {
		fmt.Printf("Error reading config file: %v\n", err)
		os.Exit(3)
	}

	// Parse the TOML file
	var config Config
	err = toml.Unmarshal(configData, &config)
	if err != nil {
		fmt.Printf("Error parsing TOML file: %v\n", err)
		os.Exit(4)
	}

	// Prepare the output snippets map
	snippets := make(map[string]SnippetEntry)

	hasError := false

	// Process each entry
	processEntry := func(entry Entry, language string) {
		snippetPath := filepath.Join(filepath.Dir(configPath), entry.Filename)
		snippetContent, err := os.ReadFile(snippetPath)
		if err != nil {
			fmt.Printf("Error reading snippet file %s: %v\n", snippetPath, err)
			hasError = true
			return
		}
		snippetLines := strings.Split(string(snippetContent), "\n")

		// Add the snippet to the map
		snippets[entry.Prefix] = SnippetEntry{
			Scope:       language,
			Prefix:      entry.Prefix,
			Body:        snippetLines,
			Description: entry.Filename,
		}
	}
	for _, entry := range config.Rust.Entries {
		processEntry(entry, "rust")
	}
	for _, entry := range config.Cpp.Entries {
		processEntry(entry, "cpp")
	}

	// Convert the snippets map to JSON
	snippetsJSON, err := json.MarshalIndent(snippets, "", "  ")
	if err != nil {
		fmt.Printf("Error marshaling snippets to JSON: %v\n", err)
		os.Exit(1)
	}

	// Write the JSON to the specified location
	outputPath := filepath.Join(filepath.Dir(configPath), config.Location)
	if strings.HasPrefix(config.Location, "/") {
		outputPath = config.Location
	}
	err = os.WriteFile(outputPath, snippetsJSON, 0644)
	if err != nil {
		fmt.Printf("Error writing snippets to file: %v\n", err)
		os.Exit(2)
	}

	fmt.Printf("Snippets successfully written to %s\n", outputPath)

	if hasError {
		fmt.Println("Some errors occurred while processing snippets.")
		os.Exit(5)
	}
}
