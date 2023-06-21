package cmd

import (
	"os"
	"regexp"
	"strings"

	"github.com/chasinglogic/projector/projects"
	"github.com/spf13/viper"
)

func matches(patterns []*regexp.Regexp, path string) bool {
	for _, rgx := range patterns {
		if rgx.MatchString(path) {
			return true
		}
	}

	return false
}

func cbWithExcludes(includes []*regexp.Regexp, excludes []*regexp.Regexp, cb projects.ProjectFunc) projects.ProjectFunc {
	return func(path string) error {
		if matches(excludes, path) && !matches(includes, path) {
			return nil
		}

		return cb(path)
	}
}

func findAllProjects(cb projects.ProjectFunc) error {
	codeDirs := viper.GetStringSlice("code_dirs")
	home, _ := os.UserHomeDir()

	excludePatterns := viper.GetStringSlice("excludes")
	includePatterns := viper.GetStringSlice("includes")

	excludes := make([]*regexp.Regexp, len(excludePatterns))
	includes := make([]*regexp.Regexp, len(includePatterns))

	for idx, pattern := range excludePatterns {
		excludes[idx] = regexp.MustCompile(pattern)
	}

	for idx, pattern := range includePatterns {
		includes[idx] = regexp.MustCompile(pattern)
	}

	for _, dir := range codeDirs {
		expanded := strings.Replace(dir, "~", home, 1)
		err := projects.Find(expanded, cbWithExcludes(includes, excludes, cb))
		if err != nil {
			return err
		}
	}

	return nil
}
