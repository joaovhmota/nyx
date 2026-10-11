package commands

import (
	"testing"

	"github.com/bwmarrin/discordgo"
)

const testBotID = "123"

func TestIsMentioned(t *testing.T) {
	mentioned := &discordgo.MessageCreate{
		Message: &discordgo.Message{
			Mentions: []*discordgo.User{{ID: testBotID}},
		},
	}
	if !isMentioned(mentioned, testBotID) {
		t.Error("expected mention to be detected")
	}

	notMentioned := &discordgo.MessageCreate{
		Message: &discordgo.Message{Content: "hello"},
	}
	if isMentioned(notMentioned, testBotID) {
		t.Error("expected mention to not be detected")
	}

	everyone := &discordgo.MessageCreate{
		Message: &discordgo.Message{MentionEveryone: true},
	}
	if !isMentioned(everyone, testBotID) {
		t.Error("expected @everyone to be treated as a mention")
	}
}

func TestIsFromBot(t *testing.T) {
	fromBot := &discordgo.Message{Author: &discordgo.User{ID: testBotID}}
	if !isFromBot(fromBot, testBotID) {
		t.Error("expected message to be from the bot")
	}

	fromUser := &discordgo.Message{Author: &discordgo.User{ID: "999"}}
	if isFromBot(fromUser, testBotID) {
		t.Error("expected message to not be from the bot")
	}

	if isFromBot(nil, testBotID) {
		t.Error("expected nil message to not be from the bot")
	}

	if isFromBot(&discordgo.Message{}, testBotID) {
		t.Error("expected message without author to not be from the bot")
	}
}

func TestStripMention(t *testing.T) {
	cases := map[string]string{
		"<@123> hello":    "hello",
		"<@!123> hello":   "hello",
		"<@123>   hello ": "hello",
		"<@123>":          "",
		"hello":           "hello",
	}

	for input, want := range cases {
		if got := stripMention(input, testBotID); got != want {
			t.Errorf("stripMention(%q) = %q, want %q", input, got, want)
		}
	}
}

func TestPromptFor(t *testing.T) {
	if got := promptFor("question"); got != "question" {
		t.Errorf("promptFor(question) = %q, want question", got)
	}

	if got := promptFor(""); got != bareMentionPrompt {
		t.Errorf("promptFor(\"\") = %q, want bareMentionPrompt", got)
	}
}
