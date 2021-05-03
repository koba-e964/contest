-module('Main').
-export([main/1]).

calc([T1, X1, Y1 | L = [T2, X2, Y2 | _]]) ->
    T = T2 - T1,
    Dist = abs(X2 - X1) + abs(Y2 - Y1),
    case Dist =< T andalso (Dist + T) rem 2 =:= 0 of
        true -> calc(L);
        false -> false
    end;
calc(_) -> true.

solve() ->
    [N] = input("~d"),
    List = input(lists:flatten(lists:duplicate(3 * N, "~d"))),
    io:format("~s~n", [case calc([0, 0, 0 | List]) of
        true -> "Yes";
        false -> "No"
    end]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
