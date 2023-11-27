package projector

import (
	"adam/pkg/config"
	"fmt"
	"os"
	"path"
)

type Opteration = int

const (
	Print Opteration = iota
	Add
	Remove
)

type Config struct {
	Args      []string
	Operation Opteration
	Pwd       string
	Config    string
}

func getPwd(opts *config.Opts) (string, error) {
	if opts.Pwd != "" {
		return opts.Pwd, nil
	}

	return os.Getwd()
}

func getConfig(opts *config.Opts) (string, error) {
	if opts.Config != "" {
		return opts.Config, nil
	}

	xdg, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return path.Join(xdg, "projector", "projects.json"), nil
}

func getOperation(opts *config.Opts) Opteration {
	if len(opts.Args) == 0 {
		return Print
	}

	if opts.Args[0] == "add" {
		return Add
	}

	if opts.Args[0] == "remove" {
		return Remove
	}

	return Print
}

func getArgs(opts *config.Opts) ([]string, error) {
	if len(opts.Args) == 0 {
		return []string{}, nil
	}

	if opts.Args[0] == "add" {
		if len(opts.Args) != 3 {
			return []string{}, fmt.Errorf("invalid number of arguments for add operation")
		}
		return opts.Args[1:], nil
	}

	if opts.Args[0] == "remove" {
		if len(opts.Args) != 2 {
			return []string{}, fmt.Errorf("invalid number of arguments for remove operation")
		}
		return opts.Args[1:], nil
	}

	if len(opts.Args) > 1 {
		return []string{}, fmt.Errorf("invalid number of arguments for print operation")
	}

	return opts.Args, nil
}

func NewConfig(opts *config.Opts) (*Config, error) {
	pwd, err := getPwd(opts)
	if err != nil {
		return nil, err
	}

	config, err := getConfig(opts)
	if err != nil {
		return nil, err
	}

	args, err := getArgs(opts)
	if err != nil {
		return nil, err
	}

	return &Config{
		Args:      args,
		Operation: getOperation(opts),
		Pwd:       pwd,
		Config:    config,
	}, nil
}
