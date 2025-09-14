package cmd

import (
	"errors"
	"fmt"
	"regexp"
	"sort"
	"strings"

	"github.com/spf13/cobra"
)

var findCmd = &cobra.Command{
	Use:     "find",
	Short:   "Find projects by matching their paths",
	Aliases: []string{"f", "search"},
	RunE: func(cmd *cobra.Command, args []string) error {
		finder, err := getFinder()
		if err != nil {
			return err
		}

		projects, err := finder.Find()
		if err != nil {
			return err
		}

		searchTerm := strings.Join(args, " ")
		rgx, err := regexp.Compile(searchTerm)
		if err != nil {
			return fmt.Errorf("invalid regex: %w", err)
		}

		var matchedProjects []string
		for _, project := range projects {
			if rgx.MatchString(project) {
				matchedProjects = append(matchedProjects, project)
			}
		}

		if len(matchedProjects) == 0 {
			return errors.New("No projects matched that search.")
		}

		if verbose {
			for _, project := range matchedProjects {
				fmt.Println(project)
			}

			return nil
		}

		result := getBestCandidate(matchedProjects, reverse, rgx)
		fmt.Println(result)
		return nil
	},
}

// getBestCandidate will determine which project in matchedProjects is the correct one
// to return to the user.
//
// It does this by either scoring the matched projects by which
// matched the regex at the rightmost position by percentage unless reverse is true in
// which case it will instead determine it by the match with leftmost position.
func getBestCandidate(matchedProjects []string, reverse bool, rgx *regexp.Regexp) string {
	sort.Slice(matchedProjects, func(i, j int) bool {
		a := matchedProjects[i]
		b := matchedProjects[j]
		matchA := float64(rgx.FindStringIndex(a)[1]) / float64(len(a))
		matchB := float64(rgx.FindStringIndex(b)[1]) / float64(len(b))

		if matchA == matchB {
			return len(a) < len(b)
		}

		return matchA < matchB
	})

	if reverse {
		return matchedProjects[0]
	}

	return matchedProjects[len(matchedProjects)-1]
}

var reverse bool

func init() {
	findCmd.Flags().BoolVarP(&reverse, "reverse", "r", false, "If provided find leftmost match instead of rightmost match")
	rootCmd.AddCommand(findCmd)
}
