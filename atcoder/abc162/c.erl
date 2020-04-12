-module('Main').
-export([main/1]).

gcd(A, 0) -> A;
gcd(A, B) -> gcd(B, A rem B).

dfs(_A, _B, [], Sum) -> Sum;
dfs(A, B, [C | Ls], Sum) ->
    dfs(A, B, Ls, Sum + gcd(gcd(A, B), C)).

solve() ->
    [K] = input("~d"),
    Ls = lists:seq(1, K),
    Sum = lists:sum([dfs(A, B, Ls, 0) || A <- Ls, B <- Ls]),
    io:format("~B~n", [Sum]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
