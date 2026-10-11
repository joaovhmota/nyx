package commands

import (
	"sync"

	"nyx/common"
)

const maxHistoryMessages = 10

type memory struct {
	mutex    sync.Mutex
	messages map[string][]common.Message
}

func newMemory() *memory {
	return &memory{messages: make(map[string][]common.Message)}
}

func (m *memory) history(channelID string) []common.Message {
	m.mutex.Lock()
	defer m.mutex.Unlock()

	return append([]common.Message(nil), m.messages[channelID]...)
}

func (m *memory) add(channelID string, messages ...common.Message) {
	m.mutex.Lock()
	defer m.mutex.Unlock()

	history := append(m.messages[channelID], messages...)
	if len(history) > maxHistoryMessages {
		history = history[len(history)-maxHistoryMessages:]
	}

	m.messages[channelID] = history
}
