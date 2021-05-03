-module('Main').
-export([main/1]).

solve() ->
    [A, B, C, X] = input("~d~d~d~d"),
    io:format("~B~n", [length(
        [ ok || I <- lists:seq(0, A), J <- lists:seq(0, B), K <- lists:seq(0, C), 500 * I + 100 * J + 50 * K =:= X]
    )]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
