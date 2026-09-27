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
	_ = godotenv.Load()
}

func main() {
	var discordToken = os.Getenv("DISCORD_BOT_TOKEN")

	if strings.TrimSpace(discordToken) == "" {
		log.Fatalln("Variable 'DISCORD_BOT_TOKEN' can't be empty, please provide one.")
	}

	var authenticationHeader = fmt.Sprintf("Bot %s", discordToken)
	var discordClient, clientError = discordgo.New(authenticationHeader)

	if clientError != nil {
		log.Fatal(clientError)
	}

	var connectionError = discordClient.Open()

	if connectionError != nil {
		log.Fatal(connectionError)
	}

	defer log.Println("Discord's client succesfully disconnected")
	defer discordClient.Close()

	log.Println("Discord's client successfully connected")

	var stop = make(chan os.Signal, 1)
	signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)

	log.Println("Press CTRL+C to shutdown")

	<-stop
}
