package log

import (
	"fmt"
	"github.com/fatih/color"
)

func Error(arguments []interface{}) string {
	msg := fmt.Sprintf("\nRuntime Error: %s\n\n", arguments[1])
	return color.RedString(msg)
}
