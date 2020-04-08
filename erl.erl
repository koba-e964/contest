-module('Main').
-export([main/1]).



solve() ->
	ok.

main(_) ->
	solve(),
	halt().

input(Pat) ->
	{ok, L} = io:fread("", Pat),
	L.