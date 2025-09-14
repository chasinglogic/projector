package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var listCmd = &cobra.Command{
	Use:     "list",
	Short:   "List all projects that projector would operate on",
	Aliases: []string{"l", "ls"},
	RunE: func(cmd *cobra.Command, args []string) error {
		finder, err := getFinder()
		if err != nil {
			return err
		}

		finder.DirtyOnly = dirty

		projects, err := finder.Find()
		if err != nil {
			return err
		}

		for _, project := range projects {
			fmt.Println(project)
		}

		return nil
	},
}

var dirty bool

func init() {
	listCmd.Flags().BoolVarP(&dirty, "dirty", "d", false, "Only show projects with a dirty git state")
	rootCmd.AddCommand(listCmd)
}
