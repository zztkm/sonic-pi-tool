package log

import (
	"fmt"
)

// msg_type to colour according to Sonic Pi GUI
// 0:     fg deeppink
// 1:     fg dodgerblue
// 2:     fg darkorange
// 3:     fg red
// 4:     fg white      bg deeppink
// 5:     fg white      bg dodgerblue
// 6:     fg white      bg darkorange
// other: fg green

type multiMessage struct {
	job_id      int32
	thread_name string
	runtime     string
	messages    []message
}

type message struct {
	msg_type int32
	info     string
}

func MultiMessage(arguments []interface{}) string {
	mm := to_multi_message(arguments)
	return mm.fmt()
}

func to_multi_message(arguments []interface{}) multiMessage {
	count := arguments[3].(int32)
	multi := multiMessage{
		job_id:      arguments[0].(int32),
		thread_name: arguments[1].(string),
		runtime:     arguments[2].(string),
		messages:    make([]message, 0),
	}
	msgs := arguments[4:]
	var i int32
	for i = 0; i < count; i++ {
		i2 := i * 2
		message := message{
			msg_type: msgs[i2].(int32),
			info:     msgs[i2+1].(string),
		}
		multi.messages = append(multi.messages, message)
	}
	return multi
}

func (mm multiMessage) fmt() string {
	out := fmt.Sprintf("\n[Run %d, Time %s]\n", mm.job_id, mm.runtime)
	max := len(mm.messages) - 1
	for i, msg := range mm.messages {
		if i < max {
			out += fmt.Sprintf(" ├ %s\n", msg.fmt())
		} else {
			out += fmt.Sprintf(" └ %s\n\n", msg.fmt())
		}
	}
	return out
}

func (msg message) fmt() string {
	return fmt.Sprintf("%s", msg.info) // Insert colour here
}
