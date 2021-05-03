-module('Main').
-export([main/1]).

rec([]) -> 0;
rec([A | Rest]) -> A - rec(Rest).

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    ASorted = lists:reverse(lists:sort(A)), % 逆順ソート
    io:format("~B~n", [rec(ASorted)]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
