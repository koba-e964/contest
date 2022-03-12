package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/fs"
	"log"
	"net/http"
	"os"
	"path"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v2"
)

// Compile: go build -o submit submit.go
// Run: ./submit PROBLEM_ID.ext or ./submit ArenaID/ID.ext
// You can just run without explicitly compiling by: go run submit.go PROBLEM_ID.ext

const Endpoint string = "https://judgeapi.u-aizu.ac.jp"
const JSessionIDName string = "JSESSIONID"

type session struct {
	JSessionID string
}

type aojConfig struct {
	UserID   string `yaml:"user_id"`
	Password string `yaml:"password"`
}

type languageConfigEntry struct {
	Extension string `yaml:"extension"`
	Name      string `yaml:"aoj_name"`
}

// baseDir must be absolute.
func readAOJConfig(baseDir string) (*aojConfig, string, error) {
	for {
		aojConfigPath := path.Join(baseDir, "aoj_config.yml")
		yamlContent, err := os.ReadFile(aojConfigPath)
		// If the file is not found, recurse into the parent directory.
		if _, ok := err.(*fs.PathError); ok {
			baseDir = path.Clean(path.Join(baseDir, ".."))
			continue
		}
		if err != nil {
			return nil, "", err
		}
		var result aojConfig
		if err := yaml.Unmarshal(yamlContent, &result); err != nil {
			return nil, "", err
		}
		return &result, baseDir, nil
	}
}

// baseDir must be absolute.
func readLanguageConfig(baseDir string) ([]languageConfigEntry, error) {
	for {
		aojConfigPath := path.Join(baseDir, "languages.yml")
		yamlContent, err := os.ReadFile(aojConfigPath)
		// If the file is not found, recurse into the parent directory.
		if _, ok := err.(*fs.PathError); ok {
			baseDir = path.Clean(path.Join(baseDir, ".."))
			continue
		}
		if err != nil {
			return nil, err
		}
		var result []languageConfigEntry
		if err := yaml.Unmarshal(yamlContent, &result); err != nil {
			return nil, err
		}
		return result, nil
	}
}

// http://developers.u-aizu.ac.jp/api?key=judgeapi%2Fsession_POST
func login(aojConfig *aojConfig) (*session, error) {
	url := Endpoint + "/session"
	jsonStream, err := json.Marshal(map[string]string{
		"id":       aojConfig.UserID,
		"password": aojConfig.Password,
	})
	if err != nil {
		return nil, err
	}
	resp, err := http.Post(url, "application/json", bytes.NewReader(jsonStream))
	if err != nil {
		return nil, err
	}
	if resp.StatusCode != 200 {
		return nil, fmt.Errorf("StatusCode = %d != 200", resp.StatusCode)
	}
	cookies := resp.Cookies()
	var jSessionID string
	for _, cookie := range cookies {
		if cookie.Name == JSessionIDName {
			jSessionID = cookie.Value
		}
	}
	if jSessionID == "" {
		return nil, fmt.Errorf("%s not found", JSessionIDName)
	}
	return &session{JSessionID: jSessionID}, nil
}

// http://developers.u-aizu.ac.jp/api?key=judgeapi%2Farenas%2F%7BarenaId%7D%2Fsubmissions_POST
func (session *session) submitInArena(arenaID, ID, language, source string) error {
	url := fmt.Sprintf("%s/arenas/%s/submissions", Endpoint, arenaID)

	client := &http.Client{}

	// example:
	// arenaId: "TUPC2021"
	// id: "D" (seems to be case insensitive, but not documented)
	// language: "C++"
	// sourceCode: "a"
	jsonStream, err := json.Marshal(map[string]string{
		"arenaId":    arenaID,
		"id":         ID,
		"language":   language,
		"sourceCode": source,
	})
	if err != nil {
		return err
	}
	req, err := http.NewRequest("POST", url, bytes.NewReader(jsonStream))
	if err != nil {
		log.Println("NewRequest failed")
		return err
	}
	req.AddCookie(&http.Cookie{
		Name:  JSessionIDName,
		Value: session.JSessionID,
	})
	req.Header.Add("Content-Type", "application/json") // Without this, the judge server fails with Internal Error.
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	if resp.StatusCode/100 != 2 {
		return fmt.Errorf("StatusCode = %d != 2xx", resp.StatusCode)
	}
	return nil
}

// http://developers.u-aizu.ac.jp/api?key=judgeapi%2Fsession_DELETE
func (session *session) logout() error {
	url := Endpoint + "/session"

	client := &http.Client{}

	req, err := http.NewRequest("DELETE", url, nil)
	if err != nil {
		log.Println("NewRequest failed")
		return err
	}
	req.AddCookie(&http.Cookie{
		Name:  JSessionIDName,
		Value: session.JSessionID,
	})
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	if resp.StatusCode/100 != 2 {
		return fmt.Errorf("StatusCode = %d != 2xx", resp.StatusCode)
	}
	return nil
}

func main() {
	if len(os.Args) != 2 {
		log.Fatalf("Usage: ./submit PROBLEM_ID.ext\n")
	}
	filename := os.Args[1]
	fileDir, err := filepath.Abs(path.Dir(filename))
	if err != nil {
		log.Fatalf("%w", err)
	}
	aojConfig, aojConfigDir, err := readAOJConfig(fileDir)
	if err != nil {
		log.Fatalf("%w", err)
	}
	log.Printf("user_id = %s", aojConfig.UserID)
	languages, err := readLanguageConfig(fileDir)
	if err != nil {
		log.Fatalf("%w", err)
	}

	// Find language and problem id
	extension := path.Ext(filename)
	if extension == "" {
		log.Fatal("\x1b[34minvalid extension\x1b[0m")
	}
	filename, err = filepath.Abs(filename)
	if err != nil {
		log.Fatalf("%w", err)
	}
	relfilename, err := filepath.Rel(aojConfigDir, filename)
	if err != nil {
		log.Fatalf("%w", err)
	}
	problemName := strings.TrimSuffix(relfilename, extension)
	var language string
	for _, entry := range languages {
		if "."+entry.Extension == extension {
			language = entry.Name
		}
	}
	if language == "" {
		log.Fatalf("Language for extension %s not found in languages.yml", extension)
	}
	log.Printf("problemName = %s", problemName)
	source, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	session, err := login(aojConfig)
	if err != nil {
		log.Fatalf("%w", err)
	}
	defer (func() {
		err = session.logout()
		if err != nil {
			log.Fatalf("%w", err)
		}
	})()
	// TODO: subtyping
	segments := strings.Split(problemName, "/")
	if len(segments) == 2 {
		arenaID := segments[0]
		ID := segments[1]
		err := session.submitInArena(arenaID, ID, language, string(source))
		if err != nil {
			log.Fatalf("%w", err)
		}
	} else if len(segments) == 1 {
		log.Panicf("TODO: not implemented")
	} else {
		log.Fatalf("Invalid relative path: %s", relfilename)
	}
}
