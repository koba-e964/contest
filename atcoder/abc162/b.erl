-module('Main').
-export([main/1]).

solve() ->
    [N] = input("~d"),
    Sum = lists:sum([A || A <- lists:seq(1, N), A rem 3 =/= 0, A rem 5 =/= 0]),
    io:format("~B~n", [Sum]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
