package main

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path"
	"regexp"
	"strings"

	"github.com/chasinglogic/projector/pkg/projects"
	"gopkg.in/yaml.v3"
)

func printProject(project string) error {
	fmt.Println(project)
	return nil
}

func findProject(searchString string) func(string) error {
	return func(project string) error {
		if strings.Contains(project, searchString) {
			fmt.Println(project)
			return errors.New("found!")
		}

		return nil
	}
}

func do() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}

	cfgFile := path.Join(homeDir, ".projector.yml")
	fh, err := os.Open(cfgFile)
	if err != nil {
		return err
	}

	var cfg Config
	if err := yaml.NewDecoder(fh).Decode(&cfg); err != nil {
		return err
	}

	excludeRgxString := ""
	for _, r := range cfg.Excludes {
		if excludeRgxString == "" {
			excludeRgxString = r
		} else {
			excludeRgxString += fmt.Sprintf("%s|%s", excludeRgxString, r)
		}
	}

	var excludes *regexp.Regexp
	if excludeRgxString != "" {
		excludes = regexp.MustCompile(excludeRgxString)
	}

	for _, root := range cfg.CodeDirs {
		if root[0] == '~' {
			root = strings.Replace(root, "~", homeDir, 1)
		}

		projects.Find(context.Background(), root, printProject, nil, excludes)
	}

	return nil
}

func main() {
	if err := do(); err != nil {
		panic(err)
	}
}
