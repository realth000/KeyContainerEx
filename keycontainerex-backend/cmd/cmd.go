package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
)

const (
	AppName    = "KeyContainerEx"
	AppVersion = "v0.1.0"
)

var rootCmd = &cobra.Command{
	Use:   AppName,
	Short: fmt.Sprintf("%s %s: Manage password offline and everywhere", AppName, AppVersion),
}

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "add password",
	Run: func(cmd *cobra.Command, args []string) {

	},
}

var removeCmd = &cobra.Command{
	Use:   "remove",
	Short: "remove password",
	Run: func(cmd *cobra.Command, args []string) {

	},
}

var showCmd = &cobra.Command{
	Use:   "show",
	Short: "show password for given account",
	Run: func(cmd *cobra.Command, args []string) {

	},
}

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "get or set config",
	Run: func(cmd *cobra.Command, args []string) {

	},
}

func init() {
	rootCmd.AddCommand(addCmd)
	rootCmd.AddCommand(removeCmd)
	rootCmd.AddCommand(showCmd)
	rootCmd.AddCommand(configCmd)
}

func Run() {
	if err := rootCmd.Execute(); err != nil {
		_, _ = fmt.Fprint(os.Stderr, err)
		os.Exit(1)
	}
}
