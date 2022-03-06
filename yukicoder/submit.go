package main

// Compile: go build -o submit submit.go
// Run: ./submit PROBLEM_ID.ext

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"mime/multipart"
	"net/http"
	"os"
	"path"
	"strings"

	"gopkg.in/yaml.v2"
)

type yukicoderConfig struct {
	ApiKey string `yaml:"api_key"`
}

type languageConfigEntry struct {
	Extension     string `yaml:"extension"`
	YukicoderName string `yaml:"yukicoder_name"`
}

func readYukicoderConfig(yukicoderConfigPath string) (*yukicoderConfig, error) {
	yamlContent, err := ioutil.ReadFile(yukicoderConfigPath)
	if err != nil {
		return nil, err
	}
	var result yukicoderConfig
	if err := yaml.Unmarshal(yamlContent, &result); err != nil {
		return nil, err
	}
	return &result, nil
}

func readLanguageConfig(languageConfigPath string) ([]languageConfigEntry, error) {
	yamlContent, err := ioutil.ReadFile(languageConfigPath)
	if err != nil {
		return nil, err
	}
	var result []languageConfigEntry
	if err := yaml.Unmarshal(yamlContent, &result); err != nil {
		return nil, err
	}
	return result, nil
}

func getProblemIDFromProblemName(problemName string) (int, error) {
	url := fmt.Sprintf("https://yukicoder.me/api/v1/problems/no/%s", problemName)
	resp, err := http.Get(url)
	if err != nil {
		return 0, err
	}
	if resp.StatusCode != 200 {
		return 0, fmt.Errorf("StatusCode = %d != 200", resp.StatusCode)
	}
	var respData struct {
		ProblemID int `json:"ProblemId"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&respData); err != nil {
		return 0, err
	}
	return respData.ProblemID, nil
}

// postCode returns SubmissionId
func postCode(problemID int, language string, code []byte, apiKey string) (uint64, error) {
	url := fmt.Sprintf("https://yukicoder.me/api/v1/problems/%d/submit", problemID)

	client := &http.Client{}

	body := &bytes.Buffer{}
	writer := multipart.NewWriter(body)
	if err := writer.WriteField("lang", language); err != nil {
		log.Println("Populating lang failed")
		return 0, err
	}
	if err := writer.WriteField("source", string(code)); err != nil {
		log.Println("Populating source failed")
		return 0, err
	}
	writer.Close()

	req, err := http.NewRequest("POST", url, body)
	if err != nil {
		log.Println("NewRequest failed")
		return 0, err
	}
	req.Header.Add("Authorization", "bearer "+apiKey)
	req.Header.Add("Accept", "application/json")
	req.Header.Add("Content-Type", writer.FormDataContentType())

	resp, err := client.Do(req)
	if err != nil {
		log.Println("Request failed")
		return 0, err
	}

	var respData struct {
		SubmissionID uint64 `json:"SubmissionId"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&respData); err != nil {
		log.Println("Parsing JSON failed")
		return 0, err
	}
	return respData.SubmissionID, nil
}

func main() {
	if len(os.Args) != 2 {
		log.Fatalf("Usage: ./submit PROBLEM_ID.ext\n")
	}

	log.Print("Start submitting...")
	filename := os.Args[1]

	exec, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}
	scriptDir := path.Dir(exec)
	yukicoderConfigPath := path.Join(scriptDir, "yukicoder_config")
	languageConfigPath := path.Join(scriptDir, "..", "languages.yml")
	yukicoderConfig, err := readYukicoderConfig(yukicoderConfigPath)
	if err != nil {
		log.Fatal(err)
	}
	languages, err := readLanguageConfig(languageConfigPath)
	if err != nil {
		log.Fatal(err)
	}

	extension := path.Ext(filename)
	if extension == "" {
		log.Fatalln("\x1b[34minvalid extension\x1b[0m")
	}
	problemName := strings.TrimSuffix(filename, extension)
	var language string
	for _, entry := range languages {
		if "."+entry.Extension == extension {
			language = entry.YukicoderName
		}
	}
	if language == "" {
		log.Fatalf("Language for extension %s not found in languages.yml", extension)
	}
	code, err := ioutil.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	problemID, err := getProblemIDFromProblemName(problemName)
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("Submitting \x1b[34m%s\x1b[0m as \x1b[34m%s\x1b[0m", filename, language)
	log.Printf("\tto \x1b[32m%s\x1b[0m (problem_id = %d)", problemName, problemID)
	submissionID, err := postCode(problemID, language, code, yukicoderConfig.ApiKey)
	if err != nil {
		log.Fatal("Submission unsuccessful: ", err)
	}
	log.Printf("Done. SubmissionId = %d", submissionID)
}
