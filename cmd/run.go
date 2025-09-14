package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var runCmd = &cobra.Command{
	Use:     "run",
	Short:   "Run a command on all matching projects",
	Aliases: []string{"x"},
	RunE: func(cmd *cobra.Command, args []string) error {
		finder, err := getFinder()
		if err != nil {
			return err
		}

		projects, err := finder.Find()
		if err != nil {
			return err
		}

		program := args[0]
		programArgs := args[1:]

		for _, project := range projects {
			fmt.Printf("\n\n%s:\n", project)
			c := exec.Command(program, programArgs...)
			c.Dir = project
			c.Stdout = os.Stdout
			c.Stderr = os.Stderr
			if err := c.Run(); err != nil {
				fmt.Println("ERROR:", err)
			}
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(runCmd)
}
