package log

import (
	"fmt"
)

func SyntaxError(arguments []interface{}) string {
	position := arguments[1]
	return fmt.Sprintf("\nSyntax Error: %s\n\n", position)
}
