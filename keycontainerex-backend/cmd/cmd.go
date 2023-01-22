package cmd

import (
	"KeyContainerEx/core"
	"KeyContainerEx/log"
	"KeyContainerEx/secure"
	"KeyContainerEx/storage"
	"KeyContainerEx/util"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/aes"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"os"
	"time"
)

var (
	defaultStoragePath = ""
	activeStorage      *storage.Storage

	showAccount   bool
	showPassword  bool
	showComment   bool
	showUseRegexp bool
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
		exist, err := core.CheckInit(storagePath)
		if err != nil {
			log.FatalPrintln("failed to check init:", err)
		}
		if !exist {
			activeStorage, err = core.Initialize(storagePath)
			if err != nil {
				log.FatalPrintln(err)
			}
			fmt.Println(activeStorage.FilePath)
			err = activeStorage.Save()
			if err != nil {
				log.ErrorPrintln("failed to save storage", err)
			}
		} else {
			activeStorage = storage.NewEmptyStorage(storagePath)
			err = activeStorage.Load()
			if err != nil {
				log.FatalPrintln("failed to load storage", err)
			}
			if false {
				err = activeStorage.MainPassword.RequireMainPassword()
				if err != nil {
					log.FatalPrintln("failed to login:", err)
				}
			}
		}
	},
}

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "add password",
	PreRun: func(cmd *cobra.Command, args []string) {
		if false {
			err := activeStorage.MainPassword.RequireMainPassword()
			if err != nil {
				log.FatalPrintln("failed to login:", err)
			}
		}
	},
	Run: func(cmd *cobra.Command, args []string) {
		var account, password, comment string
		var err error
		var pw *secure.Password
		fmt.Println("Add password")
		account, err = util.ReadStdinln("Input account: ")
		if err != nil {
			log.FatalPrintln("failed to read account from stdin:", err)
		}
		password, err = util.ReadPassword("Input password: ")
		if err != nil {
			log.FatalPrintln("failed to read password from stdin:", err)
		}
		comment, err = util.ReadStdinln("Input comment: ")
		if err != nil {
			log.FatalPrintln("failed to read comment from stdin:", err)
		}
		if account == "" || password == "" || comment == "" {
			log.FatalPrintln("can not add password: empty account, password or comment")
		}
		pw, err = secure.NewPassword(account, password, comment, *secure.NewPasswordOption(
			aes.Type256,
			aes.ModeCFB,
			activeStorage.MainPassword.GetHash(),
		))
		if err != nil {
			log.FatalPrintln("failed to add password:", err)
		}
		if err = activeStorage.AddPassword(pw); err != nil {
			log.FatalPrintln("failed to add password:", err)
		}
		if err = activeStorage.Save(); err != nil {
			log.FatalPrintln("failed to save storage:", err)
		}
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
	Args: func(cmd *cobra.Command, args []string) error {
		// Actually when user use no args, there will be an empty string in args.
		// So this len == 0 should not happen, but check this to prevent bugs.
		if len(args) == 0 || args[0] == "" {
			return fmt.Errorf("no search pattern provided")
		}
		// Default, search in account and comment.
		if !showAccount && !showComment {
			showAccount = true
			showComment = true
		}
		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		if false {
			err := activeStorage.MainPassword.RequireMainPassword()
			if err != nil {
				log.FatalPrintln("failed to login:", err)
			}
		}
		searchResult, err := activeStorage.Search(args[0], showAccount, showPassword, showComment)
		if err != nil {
			log.FatalPrintln(err)
		}
		if len(searchResult) == 0 {
			return
		}
		for _, p := range searchResult {
			// FIXME: There should be no error.
			a, _ := p.Account()
			c, _ := p.Comment()
			fmt.Printf("Account:%s\nComment:%s\n", a, c)
		}
	},
}

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "get or set config",
	Run: func(cmd *cobra.Command, args []string) {

	},
}

func init() {
	showCmd.Flags().BoolVarP(&showAccount, "account", "a", false, "search in account")
	//showCmd.Flags().BoolVarP(&showPassword, "password", "p", false, "")
	showCmd.Flags().BoolVarP(&showComment, "comment", "c", false, "search in comment")
	showCmd.Flags().BoolVarP(&showUseRegexp, "regexp", "E", false, "use search pattern is regular expression")

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
			if err = os.Mkdir(confDirPath, 0700); err != nil {
				log.Fatalln("failed to create config dir:", err)
			}
			if err = os.Mkdir(fmt.Sprintf("%s/storage", confDirPath), 0700); err != nil {
				log.Fatalln("failed to create config storage dir:", err)
			}
		} else if err == nil || os.IsExist(err) {
			if !tmpDir.IsDir() {
				if err = os.Remove(confDirPath); err != nil {
					log.Fatalln("failed to remove config file:", err)
				}
				if err = os.Mkdir(confDirPath, 0700); err != nil {
					log.Fatalln("failed to create config directory:", err)
				}
			}
			storagePath := fmt.Sprintf("%s/storage", confDirPath)
			if _, err := os.Stat(storagePath); os.IsNotExist(err) {
				if err = os.Mkdir(storagePath, 0700); err != nil {
					log.Fatalln("failed to create config storage dir:", err)
				}
			}
		} else {
			log.Fatalln("failed to check config dir:", err)
		}
		defaultStoragePath = fmt.Sprintf("%s/KeyContainerEx/storage/default.kcex", userConfDir)
	}

	if err := rootCmd.Execute(); err != nil {
		log.Fatalln("failed to execute root command:", err)
		os.Exit(1)
	}
}
