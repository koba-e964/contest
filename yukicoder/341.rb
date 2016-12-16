# coding: utf-8
res = gets.split(/[^…]+/).map(&:size).max
puts res ? res : 0
