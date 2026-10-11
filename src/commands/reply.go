package commands

import (
	"log"
	"strings"
	"time"

	"github.com/bwmarrin/discordgo"

	"nyx/common"
)

const (
	maxMessageLength = 2000
	typingInterval   = 8 * time.Second
)

type Responder interface {
	Chat(messages []common.Message) (string, error)
}

var failureMessages = map[common.Language]string{
	common.Portuguese: "Deu ruim aqui, não consegui processar isso. Tenta de novo?",
	common.English:    "Something went wrong and I couldn't process that. Try again?",
}

func replyToMessage(
	s *discordgo.Session,
	m *discordgo.MessageCreate,
	referenced *discordgo.Message,
	responder Responder,
	memory *memory,
) {
	done := make(chan struct{})
	defer close(done)

	go sendTypingLoop(s, m.ChannelID, done)

	text := stripMention(m.Content, s.State.User.ID)
	conversation := buildConversation(memory.history(m.ChannelID), referenced, s.State.User.ID, promptFor(text))

	answer, err := buildReply(responder, conversation)
	if err != nil {
		log.Printf("failed to generate reply: %v", err)

		_, _ = s.ChannelMessageSendReply(m.ChannelID, failureMessage(m.Content), m.Reference())

		return
	}

	if text != "" {
		memory.add(
			m.ChannelID,
			common.Message{Role: common.RoleUser, Content: text},
			common.Message{Role: common.RoleAssistant, Content: answer},
		)
	}

	if _, err := s.ChannelMessageSendReply(m.ChannelID, answer, m.Reference()); err != nil {
		log.Printf("failed to send reply: %v", err)
	}
}

func buildConversation(
	history []common.Message,
	referenced *discordgo.Message,
	botID string,
	question string,
) []common.Message {
	conversation := append([]common.Message(nil), history...)

	if content := referencedContent(conversation, referenced); content != "" {
		role := common.RoleUser
		if isFromBot(referenced, botID) {
			role = common.RoleAssistant
		}

		conversation = append(conversation, common.Message{Role: role, Content: content})
	}

	return append(conversation, common.Message{Role: common.RoleUser, Content: question})
}

func referencedContent(history []common.Message, referenced *discordgo.Message) string {
	if referenced == nil {
		return ""
	}

	content := strings.TrimSpace(referenced.Content)
	if content == "" {
		return ""
	}

	if len(history) > 0 && history[len(history)-1].Content == content {
		return ""
	}

	return content
}

func buildReply(responder Responder, conversation []common.Message) (string, error) {
	answer, err := responder.Chat(conversation)
	if err != nil {
		return "", err
	}

	return truncateMessage(answer), nil
}

func failureMessage(content string) string {
	return failureMessages[common.FromText(content)]
}

func sendTypingLoop(s *discordgo.Session, channelID string, done <-chan struct{}) {
	if err := s.ChannelTyping(channelID); err != nil {
		log.Printf("failed to send typing indicator: %v", err)
	}

	ticker := time.NewTicker(typingInterval)
	defer ticker.Stop()

	for {
		select {
		case <-done:
			return

		case <-ticker.C:
			if err := s.ChannelTyping(channelID); err != nil {
				log.Printf("failed to send typing indicator: %v", err)
			}
		}
	}
}

func truncateMessage(text string) string {
	const ellipsis = "…"

	runes := []rune(text)
	if len(runes) <= maxMessageLength {
		return text
	}

	return string(runes[:maxMessageLength-1]) + ellipsis
}
