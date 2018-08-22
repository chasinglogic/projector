package commands

import (
	"fmt"

	"github.com/chasinglogic/projector/projects"
	"github.com/spf13/cobra"
)

var list = &cobra.Command{
	Use:   "list",
	Short: "List projects on this system",
	Run: func(cmd *cobra.Command, args []string) {
		cfg := getConfig()
		projects.Find(cfg, func(p string) error {
			fmt.Println(p)
			return nil
		})
	},
}
