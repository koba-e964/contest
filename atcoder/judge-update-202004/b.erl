-module('Main').
-export([main/1]).


solve() ->
	Big = 100010,
	[N] = input("~d"),
	Xs = [
		begin
			[X, C] = input("~d ~s"),
			case C of
				"B" -> X + Big;
				"R" -> X
			end
		end
		|| _ <- lists:seq(1, N)
	],
	XsSorted = lists:sort(Xs),
	lists:foreach(fun (Elem) ->
		io:format("~B~n", [Elem rem Big])
	end, XsSorted),
	ok.

main(_) ->
	solve(),
	init:stop().

input(Pat) ->
	{ok, L} = io:fread("", Pat),
	L.