-module('Main').
-export([main/1]).

solve() ->
    [S] = input("~s"),
    Len = length(lists:filter(fun (Elem) -> Elem =:= $1 end, S)),
    io:format("~B~n", [Len]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
