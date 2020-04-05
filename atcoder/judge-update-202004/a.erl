-module('Main').
-export([main/1]).

main(_) ->
	{ok, [A, B, C]} = io:fread("", "~d ~d ~d"),
	io:fwrite("~p~n", [min(C,max(B,A))]),
	init:stop().