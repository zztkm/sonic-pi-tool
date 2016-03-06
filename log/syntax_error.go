package log

import (
	"fmt"
	"github.com/fatih/color"
)

func SyntaxError(arguments []interface{}) string {
	msg := fmt.Sprintf("\nSyntax Error: %s\n\n", arguments[1])
	return color.RedString(msg)
}
