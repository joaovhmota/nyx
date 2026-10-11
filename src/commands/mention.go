package commands

import (
	"strings"

	"github.com/bwmarrin/discordgo"
)

const bareMentionPrompt = "Um usuário acabou de te marcar no Discord sem falar nada. " +
	"Reaja de forma natural, direta e coloquial, perguntando o que ele precisa."

func GetMessageHandler(responder Responder) func(s *discordgo.Session, m *discordgo.MessageCreate) {
	memory := newMemory()

	return func(s *discordgo.Session, m *discordgo.MessageCreate) {
		if m.Author.ID == s.State.User.ID || m.Author.Bot {
			return
		}

		referenced := referencedMessage(s, m)
		if !respondsTo(s, m, referenced) {
			return
		}

		go replyToMessage(s, m, referenced, responder, memory)
	}
}

func respondsTo(s *discordgo.Session, m *discordgo.MessageCreate, referenced *discordgo.Message) bool {
	return isMentioned(m, s.State.User.ID) || isFromBot(referenced, s.State.User.ID)
}

func isMentioned(m *discordgo.MessageCreate, botID string) bool {
	if m.MentionEveryone {
		return true
	}

	for _, user := range m.Mentions {
		if user.ID == botID {
			return true
		}
	}

	return strings.Contains(m.Content, mentionToken(botID))
}

func isFromBot(message *discordgo.Message, botID string) bool {
	return message != nil && message.Author != nil && message.Author.ID == botID
}

func referencedMessage(s *discordgo.Session, m *discordgo.MessageCreate) *discordgo.Message {
	if m.ReferencedMessage != nil {
		return m.ReferencedMessage
	}

	if m.MessageReference == nil {
		return nil
	}

	message, err := s.ChannelMessage(m.ChannelID, m.MessageReference.MessageID)
	if err != nil {
		return nil
	}

	return message
}

func mentionToken(botID string) string {
	return "<@" + botID + ">"
}

func stripMention(content string, botID string) string {
	for _, token := range []string{
		mentionToken(botID),
		"<@!" + botID + ">",
	} {
		content = strings.ReplaceAll(content, token, "")
	}

	return strings.TrimSpace(content)
}

func promptFor(question string) string {
	if question == "" {
		return bareMentionPrompt
	}

	return question
}
