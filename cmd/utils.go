package cmd

import (
	"os"
	"strings"

	"github.com/chasinglogic/projector/projects"
	"github.com/spf13/viper"
)

func findAllProjects(cb projects.ProjectFunc) error {
	codeDirs := viper.GetStringSlice("code_dirs")
	home, _ := os.UserHomeDir()

	for _, dir := range codeDirs {
		expanded := strings.Replace(dir, "~", home, 1)
		err := projects.Find(expanded, cb)
		if err != nil {
			return err
		}
	}

	return nil
}
