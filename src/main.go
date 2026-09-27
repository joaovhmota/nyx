package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/bwmarrin/discordgo"
	"github.com/joho/godotenv"
)

func init() {
	err := godotenv.Load()

	if err != nil {
		log.Println("Could not load .env file; probably in a docker container")
	}
}

func main() {
	discordToken := os.Getenv("DISCORD_BOT_TOKEN")

	if strings.TrimSpace(discordToken) == "" {
		log.Fatalln("Variable 'DISCORD_BOT_TOKEN' can't be empty, please provide one")
	}

	authenticationHeader := fmt.Sprintf("Bot %s", discordToken)
	discordClient, clientError := discordgo.New(authenticationHeader)

	if clientError != nil {
		log.Fatal(clientError)
	}

	connectionError := discordClient.Open()

	if connectionError != nil {
		log.Fatal(connectionError)
	}

	defer log.Println("Discord's client succesfully disconnected")
	defer discordClient.Close()

	log.Println("Discord's client successfully connected")

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)

	log.Println("Press CTRL+C to shutdown")

	<-stop
}
