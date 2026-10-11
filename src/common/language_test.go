package common

import "testing"

func TestFromLocale(t *testing.T) {
	cases := map[string]Language{
		"pt-BR": Portuguese,
		"pt":    Portuguese,
		"en-US": English,
		"en":    English,
		"fr":    Portuguese,
		"":      Portuguese,
	}

	for tag, want := range cases {
		if got := FromLocale(tag); got != want {
			t.Errorf("FromLocale(%q) = %q, want %q", tag, got, want)
		}
	}
}

func TestFromText(t *testing.T) {
	cases := map[string]Language{
		"Olá, tudo bem?":            Portuguese,
		"Oi, você pode me ajudar?":  Portuguese,
		"não entendi isso":          Portuguese,
		"Hello, how are you?":       English,
		"thanks, that works for me": English,
		"":                          Portuguese,
		"12345":                     Portuguese,
	}

	for text, want := range cases {
		if got := FromText(text); got != want {
			t.Errorf("FromText(%q) = %q, want %q", text, got, want)
		}
	}
}
