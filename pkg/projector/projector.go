package projector

import (
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
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
	return hasUncommittedChanges(path) || hasUntrackedFiles(path)
}

func hasUncommittedChanges(path string) bool {
	cmd := exec.Command("git", "diff-index", "--quiet", "HEAD", "--")
	cmd.Dir = path
	err := cmd.Run()
	return err != nil
}

func hasUntrackedFiles(path string) bool {
	cmd := exec.Command("git", "ls-files", "--exclude-standard", "--others")
	cmd.Dir = path
	// ignore the error because we wanna return false anyways if there is one
	output, _ := cmd.Output()
	return len(output) > 0
}
