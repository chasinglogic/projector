package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var dirtyOnly bool

func printProject(path string) error {
	_, err := fmt.Println(path)
	return err
}

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List all projects that projector knows about",
	RunE: func(cmd *cobra.Command, args []string) error {
		return findAllProjects(printProject)
	},
}

func init() {
	listCmd.Flags().BoolVarP(&dirtyOnly, "dirty", "d", false, "Only list projects with a dirty git state.")
}
