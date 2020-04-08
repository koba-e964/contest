-module('Main').
-export([main/1]).

dfs(0, 0, 0) -> 1;
dfs(A1, A2, A3) ->
	Sum0 = case A3 of
		0 -> 0;
		_ -> dfs(A1, A2, A3 - 1)
	end,
	Sum1 = Sum0 + case A2 of
		A3 -> 0;
		_ -> dfs(A1, A2 - 1, A3)
	end,
	Sum2 = Sum1 + case A1 of
		A2 -> 0;
		_ -> dfs(A1 - 1, A2, A3)
	end,
	Sum2.

solve() ->
	[A1, A2, A3] = input("~d~d~d"),
	io:format("~B~n", [dfs(A1, A2, A3)]),
	ok.

main(_) ->
	solve(),
	halt().

input(Pat) ->
	{ok, L} = io:fread("", Pat),
	L.