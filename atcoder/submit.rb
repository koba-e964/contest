#!/usr/bin/env ruby -w
require 'nokogiri'
require 'mechanize'

# Reference: http://qiita.com/jun1_0803/items/9d3b26176b399e21fd61

if ARGV.size == 0
  puts "submit.rb (Code submitter)"
  puts "This script submits a code to AtCoder."
  puts "Run this command in the directory where code is placed."
  puts "The directory's name must be equal to the contest name."
  puts "Usage: \e[34msubmit.rb FILENAME\e[0m"
  exit 1
end

class Config
  def initialize(mod)
    @mod = mod
  end
  # Load configuration from a file. Config file should be a valid ruby script.
  def self.load_from_config_file(config_path)
    mod = Module.new
    mod.module_eval File.read(config_path)
    Config.new(mod)
  end
  def user_name
    @mod::USERNAME
  end
  def password
    @mod::PASSWORD
  end
end

# Configuration of submission
config_path = File.dirname(File.absolute_path($0)) + "/atcoder_config"
config = Config.load_from_config_file(config_path)

source_name = ARGV[0]
contest_name = File.basename(FileUtils.pwd)
task_name = source_name.split(".")[0] # x.cpp --> x
source_ext = source_name.split(".")[1] # x.cpp -> cpp
task_full_name = ""
task_id = nil
language_name = ""
language_id = ""
source = ""
open(source_name, "r") {|fp|
  source = fp.read
}

# Reads languages.yml and find the language.
script_dir = File.dirname($0) # The directory where this script is placed
lang_file = script_dir + "/../languages.yml" # Available languages
langs = YAML.load_file(lang_file)
entries = langs.select{|entry| entry['extension'] == source_ext}
if entries.size != 1
  puts 'Extension not found: ' + source_ext
  exit 1
end
source_language = entries[0]['name']

puts "Run in \e[32m#{contest_name}\e[0m"
agent = Mechanize.new

agent.get("https://atcoder.jp/login") do |page|
  page.form_with(:class => 'form-horizontal') do |form|
    form.field_with(:id => "username").value = config.user_name
    form.password = config.password
  end.submit
end



agent.get("https://atcoder.jp/contests/#{contest_name}/submit") do |page|
  doc = Nokogiri::HTML(page.content)
  problems = doc.xpath('//select[@id="select-task"]/option')
  for pr in problems
    pr_split = pr.text.split
    cur_task_name = ""
    if pr_split.size >= 1
      cur_task_name = pr_split[0].downcase # One of the trickiest part of this script: the task name MUST be of form "X - Some interesting task name"
    end
    cur_task_id = pr.attribute("value").value
    # puts "task_name = #{cur_task_name}, task_id = #{cur_task_id}"
    if task_name == cur_task_name
      task_id = cur_task_id
      task_full_name = pr.text
    end
  end
  if task_id.nil? # A proper task_id was not found
    raise "Task name #{task_name} was not found"
  end
  languages = doc.xpath("//div[@id=\"select-lang-#{task_id}\"]/select/option")
  for lang in languages
    if lang.text.index(source_language) # look for source_language in available languages
      language_name = lang.text
      language_id = lang.attribute("value").value
      break
    end
  end
  if language_name == ""
    raise "#{source_language} was not found"
  end

  csrf_token = ''
  page.form_with(:class => 'form-horizontal form-code-submit') do |form|
    csrf_token = form.field_with(:name => 'csrf_token').value
  end
  query = {
    'data.TaskScreenName' => task_id,
    'data.LanguageId' => language_id,
    'sourceCode' => source,
    'csrf_token' => csrf_token,
  }
  puts "Submitting \e[34m#{source_name}\e[0m as \e[34m#{language_name}\e[0m (id = #{language_id})"
  puts "\tto \e[32m#{task_full_name}\e[0m (id = #{task_id}) in \e[32m#{contest_name}\e[0m"
  agent.post("https://atcoder.jp/contests/#{contest_name}/submit", query)
end
agent.get("https://atcoder.jp") do |page|
  page.form_with(:name => 'form_logout').submit
end
