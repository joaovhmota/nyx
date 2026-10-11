package commands

import (
	"log"

	"github.com/bwmarrin/discordgo"

	"nyx/common"
)

var pingResponses = map[common.Language]discordgo.MessageEmbed{
	common.Portuguese: {
		Title:       "Pong",
		Description: "Estou aqui, pode falar.",
	},
	common.English: {
		Title:       "Pong",
		Description: "I'm here, go ahead.",
	},
}

func NewPingCommand() Command {
	return Command{
		Data: &discordgo.ApplicationCommand{
			Name:        "ping",
			Description: "Responde com Pong!",
			DescriptionLocalizations: &map[discordgo.Locale]string{
				discordgo.EnglishUS: "Replies with Pong!",
			},
		},
		Handler: func(s *discordgo.Session, i *discordgo.InteractionCreate) {
			err := s.InteractionRespond(i.Interaction, &discordgo.InteractionResponse{
				Type: discordgo.InteractionResponseChannelMessageWithSource,
				Data: &discordgo.InteractionResponseData{
					Embeds: []*discordgo.MessageEmbed{pingEmbed(i)},
				},
			})

			if err != nil {
				log.Printf("failed to respond to /ping: %v", err)
			}
		},
	}
}

func pingEmbed(i *discordgo.InteractionCreate) *discordgo.MessageEmbed {
	user := interactionUser(i)

	embed := pingResponses[common.FromLocale(string(i.Locale))]
	embed.Author = &discordgo.MessageEmbedAuthor{
		Name:    user.DisplayName(),
		IconURL: user.AvatarURL("1024"),
	}

	return &embed
}
