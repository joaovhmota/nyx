# Nyx

Bot de Discord que conversa com quem o menciona, usando um modelo de linguagem
servido localmente pelo [Ollama](https://ollama.com).

## Recursos

- Responde a menções com texto gerado por IA, seguindo o idioma do usuário.
- Reage quando alguém responde diretamente a uma mensagem dele.
- Mantém o contexto da conversa por canal (em memória) e considera a mensagem respondida.
- Responde ao comando `/ping`.
- Envia o indicador de "digitando" enquanto gera a resposta.

## Requisitos

- [Docker](https://docs.docker.com/get-docker/) com Docker Compose.
- Uma GPU NVIDIA com o [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
  (o serviço do Ollama reserva a GPU no `compose.yml`).
- Um bot do Discord e o seu token.

## Configuração

As variáveis de ambiente ficam em `src/.env` (veja `src/.env.example`):

| Variável            | Descrição                                    |
| ------------------- | -------------------------------------------- |
| `DISCORD_BOT_TOKEN` | Token do bot do Discord.                     |
| `OLLAMA_HOST`       | Endereço do servidor Ollama.                 |

## Como rodar

```sh
docker compose up --build
```

Na primeira execução o serviço `ollama-init` baixa o modelo `qwen3:4b-instruct`
antes de o bot iniciar.

Para encerrar, pressione `CTRL+C`.

## Rodando localmente

Com Go instalado e um servidor Ollama acessível em `OLLAMA_HOST`:

```sh
cd src
go run .
```

## Testes

```sh
cd src
go test ./...
```

## Arquitetura

```
src/
  main.go              # inicializa o cliente do Discord e registra os handlers
  commands/            # comandos slash e respostas a menções
    command.go         # tipo Command, lista de comandos e roteador
    ping.go            # comando /ping
    mention.go         # detecção de menção e de respostas
    reply.go           # geração e envio da resposta
    memory.go          # histórico da conversa por canal (em memória)
  common/
    ollama.go          # cliente do Ollama
    language.go        # detecção de idioma (pt/en)
```
