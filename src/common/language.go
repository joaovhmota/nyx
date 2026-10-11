package common

import (
	"strings"
	"unicode"
)

type Language string

const (
	Portuguese Language = "pt"
	English    Language = "en"
)

const accentCharacters = "áàâãäéèêëíìîïóòôõöúùûüçñ"

var portugueseWords = map[string]struct{}{
	"não": {}, "nao": {}, "você": {}, "voce": {}, "que": {}, "para": {},
	"pra": {}, "com": {}, "uma": {}, "como": {}, "obrigado": {}, "obrigada": {},
	"oi": {}, "olá": {}, "ola": {}, "bom": {}, "dia": {}, "tudo": {}, "bem": {},
	"por": {}, "favor": {}, "sou": {}, "está": {}, "esta": {}, "isso": {},
	"meu": {}, "minha": {}, "acho": {}, "quero": {}, "pode": {}, "fazer": {},
	"agora": {}, "aqui": {}, "muito": {}, "mais": {}, "também": {}, "tambem": {},
	"então": {}, "entao": {}, "sim": {},
}

var englishWords = map[string]struct{}{
	"the": {}, "you": {}, "and": {}, "for": {}, "with": {}, "how": {},
	"what": {}, "this": {}, "that": {}, "thanks": {}, "thank": {}, "hello": {},
	"hi": {}, "please": {}, "are": {}, "your": {}, "want": {}, "could": {},
	"would": {}, "should": {}, "very": {}, "here": {}, "is": {}, "can": {},
	"now": {}, "more": {}, "also": {}, "then": {}, "yes": {},
}

func FromLocale(tag string) Language {
	tag = strings.ToLower(tag)

	switch {
	case strings.HasPrefix(tag, "pt"):
		return Portuguese
	case strings.HasPrefix(tag, "en"):
		return English
	default:
		return Portuguese
	}
}

func FromText(text string) Language {
	lower := strings.ToLower(text)

	if strings.ContainsAny(lower, accentCharacters) {
		return Portuguese
	}

	portugueseScore, englishScore := 0, 0

	for _, word := range strings.FieldsFunc(lower, func(r rune) bool {
		return !unicode.IsLetter(r)
	}) {
		if _, ok := portugueseWords[word]; ok {
			portugueseScore++
		}

		if _, ok := englishWords[word]; ok {
			englishScore++
		}
	}

	if englishScore > portugueseScore {
		return English
	}

	return Portuguese
}
