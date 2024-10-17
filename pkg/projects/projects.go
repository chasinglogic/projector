package projects

import (
	"context"
	"os"
	"path"
	"regexp"
	"sync"
)

func Find(
	parentCtx context.Context,
	rootDir string,
	cb func(string) error,
	includes *regexp.Regexp,
	excludes *regexp.Regexp,
) {
	ctx, cancel := context.WithCancel(parentCtx)
	var projectChan = make(chan string, 10)
	var wg sync.WaitGroup

	wg.Add(1)
	go findProjects(&wg, rootDir, projectChan, excludes)

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case project := <-projectChan:
				if err := cb(project); err != nil {
					cancel()
				}
			}
		}
	}()

	wg.Wait()
	cancel()
	close(projectChan)

}

func findProjects(
	wg *sync.WaitGroup,
	rootDir string,
	projects chan string,
	excludes *regexp.Regexp,
) {
	defer wg.Done()

	entries, err := os.ReadDir(rootDir)
	if err != nil && os.IsNotExist(err) {
		return
	} else if err != nil {
		panic(err)
	}

	for _, entry := range entries {
		if entry.Name() == ".git" {
			projects <- rootDir
			return
		}
	}

	for _, entry := range entries {
		if entry.IsDir() {
			subdir := path.Join(rootDir, entry.Name())
			if excludes != nil && excludes.MatchString(subdir) {
				continue
			}

			wg.Add(1)
			go findProjects(wg, subdir, projects, excludes)
		}
	}
}
