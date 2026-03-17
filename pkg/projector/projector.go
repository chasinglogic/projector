package projector

import (
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
)

type Finder struct {
	DirtyOnly bool

	config     *Config
	candidates []string
	matches    []string
}

func NewFinder(config *Config) *Finder {
	return &Finder{
		config:     config,
		candidates: config.CodeDirs,
	}
}

func (f *Finder) Find() ([]string, error) {
	for len(f.candidates) > 0 {
		// Pop the first candidate off the stack
		candidate := f.candidates[0]
		f.candidates = f.candidates[1:]

		entries, err := os.ReadDir(candidate)
		if err != nil {
			continue
		}

		for _, entry := range entries {
			path := filepath.Join(candidate, entry.Name())
			if f.isExcluded(path) && !f.isIncluded(path) {
				continue
			}

			if entry.IsDir() {
				gitPath := filepath.Join(path, ".git")
				if _, err := os.Stat(gitPath); err == nil {
					if f.DirtyOnly && !isDirty(path) {
						continue
					}

					f.matches = append(f.matches, path)
				} else {
					f.candidates = append(f.candidates, path)
				}
			}
		}
	}
	return f.matches, nil
}

func (f *Finder) isExcluded(path string) bool {
	for _, pattern := range f.config.Excludes {
		matched, _ := regexp.MatchString(pattern, path)
		if matched {
			return true
		}
	}

	return false
}

func (f *Finder) isIncluded(path string) bool {
	for _, pattern := range f.config.Includes {
		matched, _ := regexp.MatchString(pattern, path)
		if matched {
			return true
		}
	}

	return false
}

func isDirty(path string) bool {
	cmd := exec.Command("git", "status", "--porcelain")
	cmd.Dir = path
	output, _ := cmd.Output()
	return len(strings.TrimSpace(string(output))) > 0
}
