package example_test

import (
	"testing"
)

func TestWeCanTest(t *testing.T) {
	if 1 != 1 {
		t.Error("1 should eq 1")
	}
}
