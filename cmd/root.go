package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use:   "projector",
	Short: "A code repository manager",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println(viper.GetStringSlice("code_dirs"))
	},
}

func init() {
	cobra.OnInitialize(initConfig)

	home, err := os.UserHomeDir()
	if err != nil {
		panic(err)
	}

	rootCmd.PersistentFlags().StringSliceP("excludes", "e", []string{}, "Regex to exclude results from the search set")
	viper.BindPFlag("excludes", rootCmd.PersistentFlags().Lookup("excludes"))

	rootCmd.PersistentFlags().StringSliceP("includes", "i", []string{}, "Regex to include results from the search set, overrides any exclude patterns")
	viper.BindPFlag("includes", rootCmd.PersistentFlags().Lookup("includes"))

	rootCmd.PersistentFlags().StringSliceP("code-dir", "c", []string{home}, "Directories to search for projects")
	viper.BindPFlag("code_dirs", rootCmd.PersistentFlags().Lookup("code-dir"))

	rootCmd.AddCommand(listCmd)
	rootCmd.AddCommand(findCmd)
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
