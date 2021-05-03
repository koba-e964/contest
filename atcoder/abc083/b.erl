-module('Main').
-export([main/1]).

sum(N) when N =< 0 -> 0;
sum(N) -> sum(N div 10) + N rem 10.

solve() ->
    [N, A, B] = input("~d~d~d"),
    io:format("~B~n", [lists:sum(
        [I || I <- lists:seq(1, N), A =< sum(I), sum(I) =< B]
    )]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
