#!/usr/bin/env ruby -w
require 'nokogiri'
require 'mechanize'

# Reference: http://qiita.com/jun1_0803/items/9d3b26176b399e21fd61

if ARGV.size == 0
  puts "submit.rb (Code submitter)"
  puts "This script submits a code to AtCoder."
  puts "Run this command in the directory where code is placed."
  puts "The directory's name must be equal to the contest name."
  puts "It supports only \e[32mC++\e[0m code."
  puts "Usage: \e[34msubmit.rb FILENAME\e[0m"
  exit 1
end

# Configuration of submission
config_path = File.dirname(File.absolute_path($0)) + "/atcoder_config"
mod = Module.new
mod.module_eval File.read(config_path)
source_name = ARGV[0]
contest_name = File.basename(FileUtils.pwd)
task_name = source_name.split(".")[0] # x.cpp --> x
task_full_name = ""
task_id = ""
language_name = ""
language_id = ""
source = ""
open(source_name, "r") {|fp|
  source = fp.read
}

puts "Run in \e[32m#{contest_name}\e[0m"
agent = Mechanize.new

agent.get("https://#{contest_name}.contest.atcoder.jp/login") do |page|
  page.form_with(:class => 'form-horizontal') do |form|
    form.field_with(:name => "name").value = mod::USERNAME
    form.password = mod::PASSWORD
  end.submit
end
agent.get("https://#{contest_name}.contest.atcoder.jp/submit") do |page|
  doc = Nokogiri::HTML(page.content.toutf8)
  problems = doc.xpath('//select[@name="task_id"]/option')
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
  if task_id == "" # A proper task_id was not found
    raise "Task name \e[32m#{task_name}\e[0m was not found"
  end
  languages = doc.xpath("//select[@name=\"language_id_#{task_id}\"]/option")
  for lang in languages
    if /C\+\+/.match(lang.text) # look for C++ in available languages
      language_name = lang.text
      language_id = lang.attribute("value").value
      break
    end
  end
  if language_name == ""
    raise "C++ was not found"
  end

  mypage = page.form_with(:class => 'form-horizontal') do |form|
    form.task_id = task_id
    form.field_with(:name => "language_id_#{task_id}").value = language_id
    form.source_code = source
    puts "Submitting \e[34m#{source_name}\e[0m as \e[34m#{language_name}\e[0m (id = #{language_id})"
    puts "\tto \e[32m#{task_full_name}\e[0m (id = #{task_id}) in \e[32m#{contest_name}\e[0m"
  end.submit
end
agent.get("https://#{contest_name}.contest.atcoder.jp/logout") # logout
