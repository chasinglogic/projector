package commands

import (
	"fmt"
	"os"

	"github.com/chasinglogic/projector/config"
	"github.com/spf13/cobra"
)

var (
	codeDir  string
	includes []string
	excludes []string
)

func init() {
	Root.AddCommand(list)
	Root.AddCommand(run)

	Root.Flags().StringVarP(&codeDir, "code-dir", "c", "", "Where to search for projects. This flag overrides the config file.")
	Root.Flags().StringSliceVarP(&excludes, "exclude", "e", nil, "A regex used to exclude projects from the search.")
	Root.Flags().StringSliceVarP(&includes, "include", "i", nil, "A regex used to include projects in the search, if a project matches exclude and include it is included.")
}

// Root CLI command. This should have no functionality.
var Root = &cobra.Command{
	Use:   "projector",
	Short: "Find and operate on projects",
}

func getConfig() config.Config {
	cfg, err := config.Load()
	if err != nil {
		fmt.Println("error loading config:", err)
		os.Exit(1)
	}

	if codeDir != "" {
		cfg.CodeDir = codeDir
	}

	if includes != nil {
		cfg.Includes = includes
	}

	if excludes != nil {
		cfg.Excludes = excludes
	}

	return cfg
}
