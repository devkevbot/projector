package cli_test

import (
	"testing"

	"github.com/devkevbot/projector/pkg/cli"
)

func getData() *cli.Data {
	return &cli.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"fem": "is_great",
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

func getProjector(pwd string, data *cli.Data) *cli.Projector {
	return cli.CreateProjector(
		&cli.Config{
			Args:      []string{},
			Operation: cli.Print,
			Pwd:       pwd,
			Config:    "Hello",
		},
		data,
	)
}

func test(t *testing.T, proj *cli.Projector, key, expectedValue string) {
	value, ok := proj.GetValue(key)
	if !ok {
		t.Errorf("expected to find value %q", key)
	}
	if value != expectedValue {
		t.Errorf("expected to find %q but received %q", expectedValue, value)
	}
}

func TestGetValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	test(t, proj, "foo", "bar3")
	test(t, proj, "fem", "is_great")
}

func TestSetValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	test(t, proj, "foo", "bar3")

	proj.SetValue("foo", "baz")
	test(t, proj, "foo", "baz")

	proj.SetValue("fem", "is_super_great")
	test(t, proj, "fem", "is_super_great")

	proj = getProjector("/", data)
	test(t, proj, "fem", "is_great")
}

func TestRemoveValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	test(t, proj, "foo", "bar3")

	proj.RemoveValue("foo")
	test(t, proj, "foo", "bar2")

	proj.RemoveValue("fem")
	test(t, proj, "fem", "is_great")
}
