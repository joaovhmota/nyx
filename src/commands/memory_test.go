package commands

import (
	"testing"

	"nyx/common"
)

func TestMemory(t *testing.T) {
	memory := newMemory()

	if got := memory.history("channel"); len(got) != 0 {
		t.Fatalf("history() length = %d, want 0", len(got))
	}

	for i := 0; i < maxHistoryMessages+5; i++ {
		memory.add("channel", common.Message{Role: common.RoleUser, Content: "message"})
	}

	history := memory.history("channel")
	if len(history) != maxHistoryMessages {
		t.Errorf("history() length = %d, want %d", len(history), maxHistoryMessages)
	}

	history[0].Content = "changed"

	if memory.history("channel")[0].Content == "changed" {
		t.Error("history() should return a copy of the stored messages")
	}
}
