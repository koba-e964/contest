#!/usr/bin/ruby
require "fileutils"
if ARGV.size == 0
  puts "Directory initializer (\e[34mdir_init.rb\e[0m)"
  puts "This tool makes copies of wiz.cpp (C++ template) in a specified directory."
  puts "usage: \e[34mdir_init.rb TARGET_DIR FILENAMES...\e[0m"
  puts "typical usecase: \e[34mdir_init.rb atcoder/arc000 {a..d}\e[0m"
  exit 1
end

target_dir = ARGV[0]
target_names = ARGV[1..-1]
script_dir = File.dirname($0) # The directory where this script is placed
FileUtils.mkdir_p(target_dir)
for n in target_names
  target_file = target_dir + "/" + n + ".cpp"
  if File.exists?(target_file)
    puts "dir_init.rb: The file \e[32m#{target_file}\e[0m already exists (not overwritten)"
  else
    FileUtils.cp(script_dir + "/wiz.cpp", target_file)
  end
end
exit 0
