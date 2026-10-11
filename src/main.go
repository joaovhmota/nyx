package main

import (
	"log"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/bwmarrin/discordgo"
	"github.com/joho/godotenv"

	"nyx/commands"
	"nyx/common"
)

func init() {
	if err := godotenv.Load(); err != nil {
		log.Println("Could not load .env file; probably in a docker container")
	}
}

func main() {
	discordToken := os.Getenv("DISCORD_BOT_TOKEN")

	if strings.TrimSpace(discordToken) == "" {
		log.Fatalln("Variable 'DISCORD_BOT_TOKEN' can't be empty, please provide one")
	}

	responder, err := common.NewOllamaClient()
	if err != nil {
		log.Fatalln(err)
	}

	discordClient, err := discordgo.New("Bot " + discordToken)
	if err != nil {
		log.Fatalln(err)
	}

	discordClient.AddHandler(commands.GetHandler())
	discordClient.AddHandler(commands.GetMessageHandler(responder))

	if err := discordClient.Open(); err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err := discordClient.Close(); err != nil {
			log.Println("failed to close the Discord connection:", err)
		}
	}()

	log.Println("Discord client successfully connected")

	registerCommands(discordClient)

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)

	log.Println("Press CTRL+C to shutdown")

	<-stop
}

func registerCommands(client *discordgo.Session) {
	for _, command := range commands.CommandList {
		if _, err := client.ApplicationCommandCreate(client.State.User.ID, "", command.Data); err != nil {
			log.Fatalf("cannot create /%s command: %v", command.Data.Name, err)
		}

		log.Printf("registered /%s", command.Data.Name)
	}
}
