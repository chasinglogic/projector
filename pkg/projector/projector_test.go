package projector

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

func setupTest(t *testing.T) (string, func()) {
	t.Helper()

	tempDir, err := os.MkdirTemp("", "projector-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}

	// Create a subdirectory with a git repository
	gitDir := filepath.Join(tempDir, "git-repo")
	if err := os.Mkdir(gitDir, 0755); err != nil {
		t.Fatalf("failed to create git repo dir: %v", err)
	}
	cmd := exec.Command("git", "init")
	cmd.Dir = gitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to init git repo: %v", err)
	}
	cmd = exec.Command("git", "commit", "--allow-empty", "-m", "initial commit")
	cmd.Dir = gitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to create initial commit: %v", err)
	}

	// Create a subdirectory without a git repository
	nonGitDir := filepath.Join(tempDir, "non-git-repo")
	if err := os.Mkdir(nonGitDir, 0755); err != nil {
		t.Fatalf("failed to create non-git repo dir: %v", err)
	}

	// Create a dirty git repository
	dirtyGitDir := filepath.Join(tempDir, "dirty-git-repo")
	if err := os.Mkdir(dirtyGitDir, 0755); err != nil {
		t.Fatalf("failed to create dirty git repo dir: %v", err)
	}
	cmd = exec.Command("git", "init")
	cmd.Dir = dirtyGitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to init dirty git repo: %v", err)
	}
	if err := os.WriteFile(filepath.Join(dirtyGitDir, "untracked-file.txt"), []byte("untracked"), 0644); err != nil {
		t.Fatalf("failed to create untracked file: %v", err)
	}

	return tempDir, func() {
		_ = os.RemoveAll(tempDir)
	}
}

func TestFinderFind(t *testing.T) {
	tempDir, cleanup := setupTest(t)
	defer cleanup()

	config := &Config{
		CodeDirs: []string{tempDir},
	}

	finder := NewFinder(config)
	matches, err := finder.Find()
	if err != nil {
		t.Fatalf("Find() returned an error: %v", err)
	}

	if len(matches) != 2 {
		t.Errorf("expected 2 matches, got %d", len(matches))
	}
}

func TestFinderFindDirtyOnly(t *testing.T) {
	tempDir, cleanup := setupTest(t)
	defer cleanup()

	config := &Config{
		CodeDirs: []string{tempDir},
	}

	finder := NewFinder(config)
	finder.DirtyOnly = true
	matches, err := finder.Find()
	if err != nil {
		t.Fatalf("Find() returned an error: %v", err)
	}

	if len(matches) != 1 {
		t.Errorf("expected 1 match, got %d", len(matches))
	}

	expected := filepath.Join(tempDir, "dirty-git-repo")
	if matches[0] != expected {
		t.Errorf("expected match %s, got %s", expected, matches[0])
	}
}

func TestFinderFindWithExcludes(t *testing.T) {
	tempDir, cleanup := setupTest(t)
	defer cleanup()

	config := &Config{
		CodeDirs: []string{tempDir},
		Excludes: []string{"dirty-git-repo"},
	}

	finder := NewFinder(config)
	matches, err := finder.Find()
	if err != nil {
		t.Fatalf("Find() returned an error: %v", err)
	}

	if len(matches) != 1 {
		t.Errorf("expected 1 match, got %d", len(matches))
	}

	expected := filepath.Join(tempDir, "git-repo")
	if matches[0] != expected {
		t.Errorf("expected match %s, got %s", expected, matches[0])
	}
}

func TestFinderFindWithIncludes(t *testing.T) {
	tempDir, cleanup := setupTest(t)
	defer cleanup()

	config := &Config{
		CodeDirs: []string{tempDir},
		Excludes: []string{".*"},
		Includes: []string{"dirty-git-repo"},
	}

	finder := NewFinder(config)
	matches, err := finder.Find()
	if err != nil {
		t.Fatalf("Find() returned an error: %v", err)
	}

	if len(matches) != 1 {
		t.Errorf("expected 1 match, got %d", len(matches))
	}

	expected := filepath.Join(tempDir, "dirty-git-repo")
	if matches[0] != expected {
		t.Errorf("expected match %s, got %s", expected, matches[0])
	}
}

func TestIsDirtyWithMtimeChange(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "projector-mtime-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer func() {
		_ = os.RemoveAll(tempDir)
	}()

	// Create a clean git repository
	gitDir := filepath.Join(tempDir, "git-repo")
	if err := os.Mkdir(gitDir, 0755); err != nil {
		t.Fatalf("failed to create git repo dir: %v", err)
	}
	cmd := exec.Command("git", "init")
	cmd.Dir = gitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to init git repo: %v", err)
	}
	cleanFile := filepath.Join(gitDir, "clean-file.txt")
	if err := os.WriteFile(cleanFile, []byte("clean"), 0644); err != nil {
		t.Fatalf("failed to create clean file: %v", err)
	}
	cmd = exec.Command("git", "add", "clean-file.txt")
	cmd.Dir = gitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to add clean file: %v", err)
	}
	cmd = exec.Command("git", "commit", "-m", "initial commit")
	cmd.Dir = gitDir
	if err := cmd.Run(); err != nil {
		t.Fatalf("failed to create initial commit: %v", err)
	}

	// Intentionally bump the mtime to simulate the bug scenario
	now := time.Now().Add(10 * time.Second)
	if err := os.Chtimes(cleanFile, now, now); err != nil {
		t.Fatalf("failed to change mtime: %v", err)
	}

	// The repository should NOT be considered dirty
	if isDirty(gitDir) {
		t.Errorf("expected clean repository, but it was flagged as dirty due to mtime change")
	}
}
