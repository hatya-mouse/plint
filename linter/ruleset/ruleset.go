package ruleset

type Ruleset struct {
	Name    string `yaml:"name"`
	Version int    `yaml:"version"`
	Rules   []Rule `yaml:"rules"`
}

type Rule struct {
	ID       string         `yaml:"id"`
	Check    string         `yaml:"check"`
	Severity Severity       `yaml:"severity"`
	Args     map[string]any `yaml:"args"`
}

type Severity string

const (
	Error   Severity = "error"
	Warning Severity = "warning"
	Info    Severity = "info"
)
