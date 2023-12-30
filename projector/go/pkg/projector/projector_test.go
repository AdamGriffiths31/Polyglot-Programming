package projector_test

import (
	"adam/pkg/projector"
	"testing"
)

func createData() *projector.Data {
	return &projector.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo":  "bar1",
				"test": "test1",
			},
			"/foo": {
				"foo": "bar2",
			},
			"/foo/bar": {
				"foo": "bar3",
			},
		},
	}
}

func getProjector(pwd string, data *projector.Data) *projector.Projector {
	return projector.CreateProjector(
		&projector.Config{
			Args:      []string{},
			Operation: projector.Print,
			Pwd:       pwd,
			Config:    "Test Config",
		},
		data,
	)
}

func TestGetValue(t *testing.T) {
	projector := getProjector("/foo/bar", createData())

	val, ok := projector.GetValue("foo")
	if !ok {
		t.Errorf("Expected to find value for key 'foo'")
	}
	if val != "bar3" {
		t.Errorf("Expected value 'bar3' for key 'foo', got '%s'", val)
	}
}

func TestSetValue(t *testing.T) {
	projector := getProjector("/foo/bar", createData())

	projector.SetValue("foo", "bar4")

	val, ok := projector.GetValue("foo")
	if !ok {
		t.Errorf("Expected to find value for key 'foo'")
	}
	if val != "bar4" {
		t.Errorf("Expected value 'bar4' for key 'foo', got '%s'", val)
	}
}

func TestRemoveValue(t *testing.T) {
	projector := getProjector("/", createData())

	projector.RemoveValue("test")

	val, ok := projector.GetValue("test")
	if ok {
		t.Errorf("Expected not to find value for key 'test'")
	}
	if val != "" {
		t.Errorf("Expected value '' for key 'test', got '%s'", val)
	}
}
