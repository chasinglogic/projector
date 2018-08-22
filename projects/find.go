package projects

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/chasinglogic/projector/config"
)

// FindFn is a function which performs actions on a project path
type FindFn func(projectPath string) error

// Find will find all projects starting at CodeDir filter them using excludes
// and includes then run FindFn on matching projects.
func Find(cfg config.Config, fn FindFn) {
	filterChan := make(chan string, 10)
	projectChan := make(chan string, 10)

	go filter(cfg.Excludes, cfg.Includes, filterChan, projectChan)
	go runFindFn(projectChan, fn)

	filepath.Walk(cfg.CodeDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Println("unable to walk code dir:", err)
			return err
		}

		if !info.IsDir() {
			return nil
		}

		if isProject(path) {
			filterChan <- path
			return filepath.SkipDir
		}

		return nil
	})
}

func filter(excludes, includes []string, in chan string, out chan string) {
	include, err := regexp.Compile("(" + strings.Join(includes, "|") + ")")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	exclude, err := regexp.Compile("(" + strings.Join(excludes, "|") + ")")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	for {
		project := <-in
		b := []byte(project)
		if exclude.Match(b) && !include.Match(b) {
			continue
		}

		out <- project
	}
}

func runFindFn(projects chan string, fn FindFn) {
	for {
		project := <-projects
		fn(project)
	}
}

func isProject(path string) bool {
	_, err := os.Stat(filepath.Join(path, ".git"))
	return !os.IsNotExist(err)
}
