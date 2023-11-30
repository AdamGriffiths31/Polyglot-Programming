package projector_test

import (
	"adam/pkg/config"
	"adam/pkg/projector"
	"reflect"
	"testing"
)

func getOpts(args []string) *config.Opts {
	return &config.Opts{
		Args:   args,
		Config: "",
		Pwd:    "",
	}
}

func TestConfigPrint(t *testing.T) {
	opts := getOpts([]string{})

	config, err := projector.NewConfig(opts)
	if err != nil {
		t.Errorf("Error creating config: %s", err)
	}

	if !reflect.DeepEqual([]string{}, config.Args) {
		t.Errorf("Args not empty, got: %s", config.Args)
	}
}

func TestConfigPrintKey(t *testing.T) {
	opts := getOpts([]string{"key"})

	config, err := projector.NewConfig(opts)
	if err != nil {
		t.Errorf("Error creating config: %s", err)
	}

	if !reflect.DeepEqual([]string{"key"}, config.Args) {
		t.Errorf("Args not expected, got: %s", config.Args)
	}

	if config.Operation != projector.Print {
		t.Errorf("Operation not expected, got: %v", config.Operation)
	}
}

func TestConfigAdd(t *testing.T) {
	opts := getOpts([]string{"add", "foo", "bar"})

	config, err := projector.NewConfig(opts)
	if err != nil {
		t.Errorf("Error creating config: %s", err)
	}

	if !reflect.DeepEqual([]string{"foo", "bar"}, config.Args) {
		t.Errorf("Args not expected, got: %s", config.Args)
	}

	if config.Operation != projector.Add {
		t.Errorf("Operation not expected, got: %v", config.Operation)
	}
}

func TestConfigRemove(t *testing.T) {
	opts := getOpts([]string{"remove", "foo"})

	config, err := projector.NewConfig(opts)
	if err != nil {
		t.Errorf("Error creating config: %s", err)
	}

	if !reflect.DeepEqual([]string{"foo"}, config.Args) {
		t.Errorf("Args not expected, got: %s", config.Args)
	}

	if config.Operation != projector.Remove {
		t.Errorf("Operation not expected, got: %v", config.Operation)
	}
}
