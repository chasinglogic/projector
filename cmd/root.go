package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
	codeDirs []string
)

var rootCmd = &cobra.Command{
	Use:   "projector",
	Short: "A code repository manager",
}

func init() {
	home, err := os.UserHomeDir()
	if err != nil {
		panic(err)
	}

	rootCmd.PersistentFlags().StringSliceVarP(&codeDirs, "code-dir", "c", []string{home}, "Directories to search for projects")

	rootCmd.AddCommand(listCmd)
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
