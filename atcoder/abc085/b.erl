-module('Main').
-export([main/1]).

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    io:format("~B~n", [length(lists:usort(A))]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
