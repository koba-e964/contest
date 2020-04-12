-module('Main').
-export([main/1]).

solve() ->
    [S] = input("~s"),
    Ans = case lists:any(fun (C) -> C =:= $7 end, S) of
        true -> "Yes";
        false -> "No"
    end,
    io:format("~s~n", [Ans]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
