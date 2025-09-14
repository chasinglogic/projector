package projector

import (
	"os"
	"path/filepath"

	"gopkg.in/yaml.v2"
)

type Config struct {
	CodeDirs []string `yaml:"code_dirs"`
	Excludes []string `yaml:"excludes"`
	Includes []string `yaml:"includes"`
}

func NewConfig(codeDirs []string) *Config {
	return &Config{
		CodeDirs: codeDirs,
	}
}

func GetConfig() (*Config, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}

	configFile := filepath.Join(home, ".projector.yml")
	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		codeDir := os.Getenv("CODE_DIR")
		if codeDir == "" {
			codeDir = filepath.Join(home, "Code")
		}
		return NewConfig([]string{codeDir}), nil
	}

	data, err := os.ReadFile(configFile)
	if err != nil {
		return nil, err
	}

	var config Config
	err = yaml.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}

	for i, dir := range config.CodeDirs {
		if dir[0] == '~' {
			config.CodeDirs[i] = filepath.Join(home, dir[1:])
		}
	}

	return &config, nil
}
