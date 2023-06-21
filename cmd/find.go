package cmd

import (
	"fmt"
	"regexp"

	"github.com/spf13/cobra"
)

var printAllMatches bool

type matchedProject struct {
	path         string
	matchedIndex int
}

func runFind(cmd *cobra.Command, args []string) error {
	matchedProjects := []matchedProject{}
	rgx := regexp.MustCompile(args[0])

	collectProjects := func(path string) error {
		loc := rgx.FindIndex([]byte(path))
		if loc == nil {
			return nil
		}

		if printAllMatches {
			fmt.Println(path)
		} else {
			matchedProjects = append(matchedProjects, matchedProject{
				path:         path,
				matchedIndex: loc[0],
			})
		}

		return nil
	}

	err := findAllProjects(collectProjects)
	if err != nil {
		return err
	}

	rightMostMatch := ""
	rightMostPercentage := 0.0

	for _, project := range matchedProjects {
		percentage := float64(project.matchedIndex) / float64(len(project.path))
		if percentage > rightMostPercentage {
			rightMostMatch = project.path
			rightMostPercentage = percentage
		}
	}

	// In the case of no matches or --verbose rightMostMatch will be unpopulated
	// and we don't want to print an erroneous blank line.
	if rightMostMatch != "" {
		fmt.Println(rightMostMatch)
	}

	return nil
}

var findCmd = &cobra.Command{
	Use:   "find",
	Short: "Find a single project by using a regex.",
	RunE:  runFind,
}

func init() {
	findCmd.Flags().BoolVarP(&printAllMatches, "verbose", "v", false, "If provided will print all matches instead of the rightmost match")
}
