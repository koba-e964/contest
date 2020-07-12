#!/usr/bin/env python3
import sys
import json
import os
import re
import requests
import yaml

if len(sys.argv) != 2:
    print("./submit.py PROBLEM_ID.ext")
    sys.exit(1)

filename = sys.argv[1]

script_dir = os.path.dirname(os.path.abspath(__file__))
yukicoder_config_path = os.path.join(script_dir, 'yukicoder_config')
language_config_path = os.path.join(script_dir, '..', 'languages.yml')

with open(yukicoder_config_path) as file:
    yukicoder_config = yaml.safe_load(file)

with open(language_config_path) as file:
    languages = yaml.safe_load(file)


(problem_name, extension) = os.path.splitext(filename)
if extension == '':
    print("\x1b[34minvalid extension\x1b[0m")
    sys.exit(1)

lang = next(x for x in languages if x['extension'] == extension[1:])['yukicoder_name']
with open(filename) as file:
    source = file.read()

api_key = yukicoder_config['api_key']

url = "https://yukicoder.me/api/v1/problems/no/{}".format(problem_name)
resp = requests.get(url, headers={
    'Authorization': "bearer " + api_key,
    'Accept': 'application/json'
})
response = json.loads(resp.text)

# Find problem_id.
if 'ProblemId' not in response:
    print('Invalid response: {}', response)
    sys.exit(2)
problem_id = response['ProblemId']

post_url = "https://yukicoder.me/api/v1/problems/{}/submit".format(problem_id)

print("Submitting \x1b[34m{}\x1b[0m as \x1b[34m{}\x1b[0m".format(filename, lang))
print("\tto \x1b[32m{}\x1b[0m (problem_id = {})".format(problem_name, problem_id))

resp = requests.post(post_url, {
    'lang': lang, 'source': source,
}, headers={
    'Authorization': "bearer " + api_key,
    'Accept': 'application/json'
})

response = json.loads(resp.text)

print(response)

if 'SubmissionId' not in response:
    print(response)
    print("\x1b[34mSubmission unsuccessful\x1b[0m")
    sys.exit(1)
