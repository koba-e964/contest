-module('Main').
-export([main/1]).



solve() ->
	ok.

main(_) ->
	solve(),
	init:stop().

input(Pat) ->
	{ok, L} = io:fread("", Pat),
	L.