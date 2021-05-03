-module('Main').
-export([main/1]).

rec(A, N) when A rem 2 =:= 0 -> rec(A div 2, N + 1);
rec(_, N) -> N.

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    And = lists:foldl(fun (Elem, Acc) -> Elem bor Acc end, 0, A),
    io:format("~B~n", [rec(And, 0)]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
