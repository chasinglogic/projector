package projects

import (
	"errors"
	"os"
	"path"
)

var Stop = errors.New("find has been cancelled")

type ProjectFunc func(string) error

func isProject(dir string) bool {
	_, err := os.Stat(path.Join(dir, ".git"))
	return !os.IsNotExist(err)
}

func Find(root string, cb ProjectFunc) error {
	if isProject(root) {
		err := cb(root)
		return err
	}

	entries, err := os.ReadDir(root)
	if err != nil && !os.IsNotExist(err) {
		return err
	} else if err != nil {
		return nil
	}

	for _, entry := range entries {
		if entry.IsDir() {
			err := Find(path.Join(root, entry.Name()), cb)
			if err != nil {
				return err
			}
		}
	}

	return nil
}
