package config

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	"github.com/BurntSushi/toml"
)

// Config describes how and where to find projects
type Config struct {
	CodeDir  string
	Excludes []string
	Includes []string
}

var configLocations = []string{
	".projector.toml",
	filepath.Join(os.Getenv("HOME"), ".projector.toml"),
	filepath.Join(os.Getenv("APPDATA"), "Projector.toml"),
}

func defaultConfig() Config {
	return Config{
		CodeDir:  filepath.Join(os.Getenv("HOME"), "Code"),
		Includes: make([]string, 0),
		Excludes: make([]string, 0),
	}
}

// Load will find the config file and load it
func Load() (Config, error) {
	var cfg Config

	for _, location := range configLocations {
		_, err := os.Stat(location)
		if err != nil && os.IsNotExist(err) {
			continue
		} else if err != nil {
			return cfg, err
		}

		content, err := ioutil.ReadFile(location)
		if err != nil {
			return cfg, err
		}

		if err := toml.Unmarshal(content, &cfg); err != nil {
			return cfg, err
		}

		if strings.HasPrefix(cfg.CodeDir, "~") {
			cfg.CodeDir = strings.Replace(cfg.CodeDir, "~", os.Getenv("HOME"), 1)
		}

		return cfg, nil
	}

	return defaultConfig(), nil
}
