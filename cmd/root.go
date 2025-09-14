package cmd

import (
	"fmt"
	"os"

	"github.com/chasinglogic/projector/pkg/projector"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "projector",
	Short: "A code repository manager",
	Long:  `A longer description that spans multiple lines and likely contains examples and usage of using your application.`,
	Run: func(cmd *cobra.Command, args []string) {
		// Do Stuff Here
	},
}

var (
	excludes []string
	includes []string
	verbose  bool
	codeDir  []string
)

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().StringSliceVarP(&excludes, "excludes", "e", []string{}, "Globs to exclude from the search")
	rootCmd.PersistentFlags().StringSliceVarP(&includes, "includes", "i", []string{}, "Globs to include in the search")
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Enable verbose logging")
	rootCmd.PersistentFlags().StringSliceVarP(&codeDir, "code-dir", "c", []string{}, "The directory to search for projects in")
}

func getFinder() (*projector.Finder, error) {
	config, err := projector.GetConfig()

	if len(codeDir) > 0 {
		config.CodeDirs = codeDir
	}

	if len(excludes) > 0 {
		config.Excludes = excludes
	}

	if len(includes) > 0 {
		config.Includes = includes
	}

	if err != nil {
		return nil, err
	}

	return projector.NewFinder(config), nil
}
