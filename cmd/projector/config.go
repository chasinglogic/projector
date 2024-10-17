package main

type Config struct {
	CodeDirs []string `yaml:"code_dirs"`
	Excludes []string `yaml:"excludes"`
	Includes []string `yaml:"includes"`
}
