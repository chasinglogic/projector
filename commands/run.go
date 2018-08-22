package commands

import (
	"fmt"
	"os/exec"

	"github.com/chasinglogic/projector/projects"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var run = &cobra.Command{
	Use:   "run",
	Short: "Run a command in all projects found",
	Run: func(cmd *cobra.Command, args []string) {
		cfg := getConfig()
		bold := color.New(color.Bold).SprintFunc()
		projects.Find(cfg, func(p string) error {
			subCommand := exec.Command(args[0], args[1:]...)
			subCommand.Dir = p

			output, err := subCommand.CombinedOutput()
			if err != nil {
				fmt.Println("error running command:", err)
				return nil
			}

			fmt.Printf("%s:\n%s\n", bold(p), string(output))
			return nil
		})
	},
}
