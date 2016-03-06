package log

import (
	"fmt"
)

func MultiMessage(arguments []interface{}) string {
	run := arguments[0]
	time := arguments[2]
	info := arguments[5]
	return fmt.Sprintf(
		"\n[Run %d, Time %s]\n └ %s\n", run, time, info,
	)
}
