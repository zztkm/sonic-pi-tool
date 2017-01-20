package log

import (
	"fmt"
)

func Info(arguments []interface{}) string {
	return fmt.Sprintf("=> %s\n", arguments[0])
}
