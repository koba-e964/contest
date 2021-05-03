-module('Main').
-export([main/1]).

solve() ->
    [A, B] = input("~d~d"),
    io:format("~s~n", [case A * B rem 2 of
        0 -> "Even"; 1 -> "Odd" end]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
