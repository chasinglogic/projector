package cmd

import (
	"regexp"
	"testing"
)

func TestGetBestCandidate_RightmostMatch(t *testing.T) {
	rgx := regexp.MustCompile("taskforge")
	matchedProjects := []string{
		"/home/user/Code/taskforge/cli",
		"/home/user/Code/taskforge/taskforge",
	}

	result := getBestCandidate(matchedProjects, false, rgx)
	expected := "/home/user/Code/taskforge/taskforge"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

func TestGetBestCandidate_ReverseLeftmostMatch(t *testing.T) {
	rgx := regexp.MustCompile("taskforge")
	matchedProjects := []string{
		"/home/user/Code/taskforge/cli",
		"/home/user/Code/taskforge/taskforge",
	}

	result := getBestCandidate(matchedProjects, true, rgx)
	// Both have "taskforge" ending at position 22.
	// /home/user/Code/taskforge/cli: 22/28 = 0.786
	// /home/user/Code/taskforge/taskforge: 22/34 = 0.647
	// Ascending sort puts lower ratio first, reverse returns first element.
	expected := "/home/user/Code/taskforge/taskforge"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

func TestGetBestCandidate_ShorterPathTiebreaker(t *testing.T) {
	rgx := regexp.MustCompile("foo")
	matchedProjects := []string{
		"/x/foo",
		"/x/foo/bar",
	}

	result := getBestCandidate(matchedProjects, false, rgx)
	// /x/foo: ratio = 6/6 = 1.0
	// /x/foo/bar: ratio = 6/10 = 0.6
	// /x/foo wins (higher ratio)
	expected := "/x/foo"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

func TestGetBestCandidate_LastMatchPosition(t *testing.T) {
	rgx := regexp.MustCompile("go")
	matchedProjects := []string{
		"/home/user/gocode/gocode",
		"/home/user/gocode",
	}

	result := getBestCandidate(matchedProjects, false, rgx)
	// /home/user/gocode: last "go" ends at 13, ratio = 13/17 = 0.765
	// /home/user/gocode/gocode: last "go" ends at 20, ratio = 20/24 = 0.833
	// /home/user/gocode/gocode wins (higher ratio)
	expected := "/home/user/gocode/gocode"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}
