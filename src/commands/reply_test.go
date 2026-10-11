package commands

import (
	"errors"
	"strings"
	"testing"

	"github.com/bwmarrin/discordgo"

	"nyx/common"
)

type fakeResponder struct {
	answer       string
	err          error
	conversation []common.Message
}

func (f *fakeResponder) Chat(messages []common.Message) (string, error) {
	f.conversation = messages

	return f.answer, f.err
}

func TestBuildReply(t *testing.T) {
	responder := &fakeResponder{answer: "hi"}

	reply, err := buildReply(responder, []common.Message{{Role: common.RoleUser, Content: "prompt"}})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if reply != "hi" {
		t.Errorf("buildReply() = %q, want %q", reply, "hi")
	}

	wantErr := errors.New("boom")

	if _, err := buildReply(&fakeResponder{err: wantErr}, nil); !errors.Is(err, wantErr) {
		t.Errorf("buildReply() error = %v, want %v", err, wantErr)
	}
}

func TestBuildConversation(t *testing.T) {
	history := []common.Message{{Role: common.RoleUser, Content: "old"}}

	conversation := buildConversation(history, nil, testBotID, "question")
	if len(conversation) != 2 {
		t.Fatalf("buildConversation() length = %d, want 2", len(conversation))
	}

	if conversation[1].Role != common.RoleUser || conversation[1].Content != "question" {
		t.Errorf("buildConversation() last message = %+v, want the question", conversation[1])
	}

	referenced := &discordgo.Message{Content: "bot answer", Author: &discordgo.User{ID: testBotID}}

	conversation = buildConversation(nil, referenced, testBotID, "question")
	if len(conversation) != 2 {
		t.Fatalf("buildConversation() length = %d, want 2", len(conversation))
	}

	if conversation[0].Role != common.RoleAssistant || conversation[0].Content != "bot answer" {
		t.Errorf("buildConversation() first message = %+v, want the bot answer as assistant", conversation[0])
	}

	alreadyKnown := []common.Message{{Role: common.RoleAssistant, Content: "bot answer"}}

	conversation = buildConversation(alreadyKnown, referenced, testBotID, "question")
	if len(conversation) != 2 {
		t.Errorf("buildConversation() length = %d, want 2 (referenced message already in history)", len(conversation))
	}
}

func TestTruncateMessage(t *testing.T) {
	const ellipsis = "…"

	short := "hello"

	if got := truncateMessage(short); got != short {
		t.Errorf("truncateMessage(%q) = %q, want unchanged", short, got)
	}

	long := strings.Repeat("a", maxMessageLength+50)

	got := truncateMessage(long)
	if len([]rune(got)) != maxMessageLength {
		t.Errorf("truncateMessage() rune length = %d, want %d", len([]rune(got)), maxMessageLength)
	}

	if !strings.HasSuffix(got, ellipsis) {
		t.Errorf("truncateMessage() = %q, want suffix %q", got, ellipsis)
	}

	emojis := strings.Repeat("🏓", maxMessageLength+10)

	got = truncateMessage(emojis)
	if len([]rune(got)) != maxMessageLength {
		t.Errorf("truncateMessage() rune length = %d, want %d", len([]rune(got)), maxMessageLength)
	}

	if !strings.HasSuffix(got, ellipsis) {
		t.Errorf("truncateMessage() with emojis = %q, want suffix %q", got, ellipsis)
	}
}

func TestFailureMessage(t *testing.T) {
	if got := failureMessage("Olá, tudo bem?"); got != failureMessages[common.Portuguese] {
		t.Errorf("failureMessage(PT) = %q, want the Portuguese message", got)
	}

	if got := failureMessage("hello, how are you?"); got != failureMessages[common.English] {
		t.Errorf("failureMessage(EN) = %q, want the English message", got)
	}
}
