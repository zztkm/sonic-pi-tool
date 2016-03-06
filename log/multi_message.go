package log

import (
	"fmt"
	"github.com/fatih/color"
)

func MultiMessage(arguments []interface{}) string {
	run := arguments[0]
	time := arguments[2]
	info := arguments[5].(string)
	line1 := fmt.Sprintf("\n[Run %d, Time %s]\n", run, time)
	line2 := fmt.Sprintf(" └ %s\n", color.MagentaString(info))
	return line1 + line2
}
