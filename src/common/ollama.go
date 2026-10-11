package common

import (
	"context"

	"github.com/ollama/ollama/api"
)

const (
	model = "qwen3:4b-instruct"

	systemPrompt = "Você é a Nyx, uma assistente de IA de um servidor do Discord. " +
		"Tenha uma personalidade séria e direta, mas fale de forma bem coloquial, como em uma conversa entre amigos. " +
		"Não use emojis e travessões. " +
		"Seja concisa: responda apenas o que foi perguntado, sem introduções, resumos ou conclusões desnecessárias. " +
		"Prefira respostas curtas e vá direto ao ponto. " +
		"Responda sempre no mesmo idioma que o usuário usar."
)

type Role string

const (
	RoleUser      Role = "user"
	RoleAssistant Role = "assistant"
)

type Message struct {
	Role    Role
	Content string
}

type OllamaClient struct {
	client *api.Client
}

func NewOllamaClient() (*OllamaClient, error) {
	client, err := api.ClientFromEnvironment()

	if err != nil {
		return nil, err
	}

	return &OllamaClient{client: client}, nil
}

func (c *OllamaClient) Chat(messages []Message) (string, error) {
	stream := false

	chatMessages := make([]api.Message, 0, len(messages)+1)
	chatMessages = append(chatMessages, api.Message{Role: "system", Content: systemPrompt})

	for _, message := range messages {
		chatMessages = append(chatMessages, api.Message{
			Role:    string(message.Role),
			Content: message.Content,
		})
	}

	request := &api.ChatRequest{
		Model:    model,
		Messages: chatMessages,
		Stream:   &stream,
	}

	var answer string

	err := c.client.Chat(
		context.Background(),
		request,
		func(response api.ChatResponse) error {
			answer = response.Message.Content

			return nil
		},
	)

	if err != nil {
		return "", err
	}

	return answer, nil
}
