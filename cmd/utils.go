package cmd

import "github.com/chasinglogic/projector/projects"

func findAllProjects(cb projects.ProjectFunc) error {
	for _, dir := range codeDirs {
		err := projects.Find(dir, cb)
		if err != nil {
			return err
		}
	}

	return nil
}
