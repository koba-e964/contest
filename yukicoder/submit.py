#!/usr/bin/env python3
import os
import re
import requests
import sys
import yaml

if len(sys.argv) != 2:
    print("./submit.py PROBLEM_ID.ext")
    sys.exit(1)

filename = sys.argv[1]

script_dir = os.path.dirname(os.path.abspath(__file__))
yukicoder_config_path = os.path.join(script_dir, 'yukicoder_config')
language_config_path = os.path.join(script_dir, '..', 'languages.yml')

with open(yukicoder_config_path) as file:
    info = yaml.safe_load(file)

with open(language_config_path) as file:
    languages = yaml.safe_load(file)


(problem_name, extension) = os.path.splitext(filename)
if extension == '':
    print("\x1b[34minvalid extension\x1b[0m")
    sys.exit(1)

lang = next(x for x in languages if x['extension'] == extension[1:])['yukicoder_name']
with open(filename) as file:
    source = file.read()

cookies = info['cookies']

url = "https://yukicoder.me/problems/no/{}".format(problem_name)
response = requests.get(url, cookies=cookies)

# Find a javascript code fragment of the following form:
# ```
# var csrf_token = "xxxx";
#
csrf_token = re.search("var csrf_token = \"(.*)\"", response.text).group(1)

# Find `/problems/{}/submit`.
problem_id = re.search("/problems/([0-9]*)/submit", response.text).group(1)

post_url = "https://yukicoder.me/problems/{}/submit".format(problem_id)


print("Submitting \x1b[34m{}\x1b[0m as \x1b[34m{}\x1b[0m".format(filename, lang))
print("\tto \x1b[32m{}\x1b[0m (problem_id = {})".format(problem_name, problem_id))

resp = requests.post(post_url, {
    'csrf_token': csrf_token, 'lang': lang, '': 'on', 'source': source,
}, cookies=cookies)

submission_successful = '提出しました。'
if not re.search(submission_successful, resp.text):
    print("\x1b[34submission unsuccessful\x1b[0m")
    exit(1)
