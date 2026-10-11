package commands

import (
	"log"

	"github.com/bwmarrin/discordgo"
)

type Command struct {
	Data    *discordgo.ApplicationCommand
	Handler func(s *discordgo.Session, i *discordgo.InteractionCreate)
}

var CommandList = []Command{
	NewPingCommand(),
}

func GetHandler() func(s *discordgo.Session, i *discordgo.InteractionCreate) {
	return func(s *discordgo.Session, i *discordgo.InteractionCreate) {
		if i.Type != discordgo.InteractionApplicationCommand {
			return
		}

		user := interactionUser(i)
		commandData := i.ApplicationCommandData()

		log.Printf("%s (%s) requested to run '%s'", user.GlobalName, user.ID, commandData.Name)

		for _, command := range CommandList {
			if commandData.Name == command.Data.Name {
				command.Handler(s, i)

				return
			}
		}
	}
}

func interactionUser(i *discordgo.InteractionCreate) *discordgo.User {
	if i.Member != nil {
		return i.Member.User
	}

	return i.User
}
