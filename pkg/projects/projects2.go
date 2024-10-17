package projects

// import (
// 	"context"
// 	"os"
// 	"path"
// 	"regexp"
// 	"sync"
// )

// func Find(
// 	parentCtx context.Context,
// 	rootDir string,
// 	cb func(string) error,
// 	includes *regexp.Regexp,
// 	excludes *regexp.Regexp,
// ) {
// 	ctx, cancel := context.WithCancel(parentCtx)
// 	var projectChan = make(chan string, 10)

// 	go func() {
// 		for {
// 			select {
// 			case <-ctx.Done():
// 				return
// 			case project := <-projectChan:
// 				if err := cb(project); err != nil {
// 					cancel()
// 				}
// 			}
// 		}
// 	}()

// 	var wg sync.WaitGroup
// 	wg.Add(1)
// 	go findProjects(&wg, rootDir, projectChan, includes, excludes)
// 	wg.Wait()

// 	cancel()
// }

// func findProjects(
// 	wg *sync.WaitGroup,
// 	rootDir string,
// 	projects chan string,
// 	includes *regexp.Regexp,
// 	excludes *regexp.Regexp,
// ) {
// 	defer wg.Done()

// 	entries, err := os.ReadDir(rootDir)
// 	if err != nil && os.IsNotExist(err) {
// 		return
// 	} else if err != nil {
// 		panic(err)
// 	}

// 	found := false
// 	for _, entry := range entries {
// 		if entry.IsDir() {
// 			child := path.Join(rootDir, entry.Name())
// 			lookAhead := path.Join(child, ".git")
// 			if _, err := os.Stat(lookAhead); err == nil {
// 				projects <- child
// 				found = true
// 			}
// 		}
// 	}

// 	if found {
// 		return
// 	}

// 	for _, entry := range entries {
// 		if entry.IsDir() {
// 			subdir := path.Join(rootDir, entry.Name())
// 			shouldExclude := excludes != nil && excludes.MatchString(subdir)
// 			shouldInclude := includes != nil && includes.MatchString(subdir)
// 			if shouldExclude && !shouldInclude {
// 				continue
// 			}

// 			wg.Add(1)
// 			go findProjects(wg, subdir, projects, includes, excludes)
// 		}
// 	}
// }
