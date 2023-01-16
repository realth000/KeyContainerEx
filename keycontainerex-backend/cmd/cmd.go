package cmd

import (
	"KeyContainerEx/log"
	"fmt"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"os"
	"time"
)

var (
	defaultStoragePath = ""
)

const (
	AppName    = "KeyContainerEx"
	AppVersion = "v0.1.0"
)

var rootCmd = &cobra.Command{
	Use:   AppName,
	Short: fmt.Sprintf("%s %s: Manage password offline and everywhere", AppName, AppVersion),
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		storagePath := ""
		// Load config environment.

		// Initialize storage.
		if storagePath == "" {
			storagePath = defaultStoragePath
		}
		exist, err := checkInit(storagePath)
		if err != nil {
			fmt.Println("failed to check init:", err)
		}
		if !exist {
			err = initialize(storagePath)
			if err != nil {
			}
		}
		fmt.Println("root cmd persist pre run. exist =", exist)
	},
}

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "add password",
	PreRun: func(cmd *cobra.Command, args []string) {

		fmt.Println("add cmd pre run")
	},
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("add cmd run")

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
	cacheConfDir, err := os.UserCacheDir()
	if err != nil {
		log.DryErrorln("failed to get cache directory:", cacheConfDir)
	} else {
		logDirPath := fmt.Sprintf("%s/KeyContainerEx", cacheConfDir)
		tmpDir, err := os.Stat(logDirPath)
		if os.IsNotExist(err) {
			if err = os.Mkdir(logDirPath, 0755); err != nil {
				log.DryErrorln("failed to create log file directory")
			}
		} else if err == nil || os.IsExist(err) {
			if !tmpDir.IsDir() {
				if err = os.Remove(logDirPath); err != nil {
					log.DryErrorln("failed to delete file:", err)
				}
				if err = os.Mkdir(logDirPath, 0755); err != nil {
					log.DryErrorln("failed to create log file directory")
				}
			}
		}
		log.InitLogger(logrus.DebugLevel, fmt.Sprintf("%s/KeyContainerEx/log-%s.txt", cacheConfDir, time.Now().Format("20060102150405")))
	}

	userConfDir, err := os.UserConfigDir()
	if err != nil {
		fmt.Println("failed to get user config directory:", err)
	} else {
		confDirPath := fmt.Sprintf("%s/KeyContainerEx", userConfDir)
		tmpDir, err := os.Stat(confDirPath)
		if os.IsNotExist(err) {
			if err = os.Mkdir(confDirPath, 0755); err != nil {
				log.Fatalln("failed to create config dir:", err)
			}
		} else if err == nil || os.IsExist(err) {
			if !tmpDir.IsDir() {
				if err = os.Remove(confDirPath); err != nil {
					log.Fatalln("failed to remove config file:", err)
				}
				if err = os.Mkdir(confDirPath, 0755); err != nil {
					log.Fatalln("failed to create config directory:", err)
				}
			}
		} else {
			log.Fatalln("failed to check config dir:", err)
		}
		defaultStoragePath = fmt.Sprintf("%s/KeyContainerEx/storage/default.kcex", userConfDir)
	}

	if err := rootCmd.Execute(); err != nil {
		_, _ = fmt.Fprint(os.Stderr, err)
		os.Exit(1)
	}
}
