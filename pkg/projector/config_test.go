package projector

import (
	"os"
	"path/filepath"
	"testing"
)

func TestGetConfig(t *testing.T) {
	homeDir, err := os.MkdirTemp("", "projector-test-home")
	if err != nil {
		t.Fatalf("failed to create temp home dir: %v", err)
	}
	defer os.RemoveAll(homeDir)

	t.Setenv("HOME", homeDir)

	configFile := filepath.Join(homeDir, ".projector.yml")
	configData := `
code_dirs:
  - ~/Projects
  - /tmp
`
	if err := os.WriteFile(configFile, []byte(configData), 0644); err != nil {
		t.Fatalf("failed to write config file: %v", err)
	}

	config, err := GetConfig()
	if err != nil {
		t.Fatalf("GetConfig() returned an error: %v", err)
	}

	if len(config.CodeDirs) != 2 {
		t.Errorf("expected 2 code dirs, got %d", len(config.CodeDirs))
	}

	expectedDir1 := filepath.Join(homeDir, "Projects")
	if config.CodeDirs[0] != expectedDir1 {
		t.Errorf("expected code dir %s, got %s", expectedDir1, config.CodeDirs[0])
	}

	if config.CodeDirs[1] != "/tmp" {
		t.Errorf("expected code dir /tmp, got %s", config.CodeDirs[1])
	}
}

func TestGetConfigNoFile(t *testing.T) {
	homeDir, err := os.MkdirTemp("", "projector-test-home")
	if err != nil {
		t.Fatalf("failed to create temp home dir: %v", err)
	}
	defer os.RemoveAll(homeDir)

	t.Setenv("HOME", homeDir)
	t.Setenv("CODE_DIR", "/tmp/code")

	config, err := GetConfig()
	if err != nil {
		t.Fatalf("GetConfig() returned an error: %v", err)
	}

	if len(config.CodeDirs) != 1 {
		t.Errorf("expected 1 code dir, got %d", len(config.CodeDirs))
	}

	if config.CodeDirs[0] != "/tmp/code" {
		t.Errorf("expected code dir /tmp/code, got %s", config.CodeDirs[0])
	}
}

func TestGetConfigNoFileNoEnv(t *testing.T) {
	homeDir, err := os.MkdirTemp("", "projector-test-home")
	if err != nil {
		t.Fatalf("failed to create temp home dir: %v", err)
	}
	defer os.RemoveAll(homeDir)

	t.Setenv("HOME", homeDir)
	os.Unsetenv("CODE_DIR")

	config, err := GetConfig()
	if err != nil {
		t.Fatalf("GetConfig() returned an error: %v", err)
	}

	if len(config.CodeDirs) != 1 {
		t.Errorf("expected 1 code dir, got %d", len(config.CodeDirs))
	}

	expectedDir := filepath.Join(homeDir, "Code")
	if config.CodeDirs[0] != expectedDir {
		t.Errorf("expected code dir %s, got %s", expectedDir, config.CodeDirs[0])
	}
}
