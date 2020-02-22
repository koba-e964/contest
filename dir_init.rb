#!/usr/bin/ruby
require 'fileutils'
require 'yaml'

if ARGV.size == 0
  puts "Directory initializer (\e[34mdir_init.rb\e[0m)"
  puts "This tool makes copies of a template file (\e[34mwiz.cpp\e[0m by default) in \e[34mTARGET_DIR\e[0m."
  puts "usage: \e[32m[SOURCE=(LANGUAGE)] \e[34mdir_init.rb TARGET_DIR FILENAMES...\e[0m"
  puts "typical usecase: \e[34mSOURCE=C++ \e[34mdir_init.rb atcoder/arc000 {a..d}\e[0m"
  puts "If environment variable SOURCE is set, the template file for the language specified by it is copied, instead of wiz.cpp"
  exit 1
end

script_dir = File.dirname($0) # The directory where this script is placed
lang_file = script_dir + "/languages.yml" # Available languages
langs = YAML.load_file(lang_file)

target_dir = ARGV[0]
target_names = ARGV[1..-1]
source_lang = ENV["SOURCE"] || "C++"
entries = langs.select{|entry| entry['name'] == source_lang}
if entries.size != 1
  puts 'not found: ' + source_lang
  exit 1
end

entry = entries[0]

source_file = entry['file']
target_ext = entry['extension']
FileUtils.mkdir_p(target_dir)
for n in target_names
  target_file = target_dir + "/" + n + "." + target_ext
  if File.exists?(target_file)
    puts "dir_init.rb: The file \e[32m#{target_file}\e[0m already exists (not overwritten)"
  else
    FileUtils.cp(script_dir + "/#{source_file}", target_file)
  end
end
exit 0
