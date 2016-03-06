package log

import (
	"fmt"
)

func Error(arguments []interface{}) string {
	return fmt.Sprintf("\nRuntime Error: %s\n\n", arguments[1])
}
