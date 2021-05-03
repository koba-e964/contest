-module('Main').
-export([main/1]).

check(A, B, C, N) ->
    case 10000 * A + 5000 * B + 1000 * C =:= N of
        true -> throw({A, B, C});
        false -> ok
    end.

solve() ->
    [N, Y] = input("~d~d"),
    {A, B, C} = try
        lists:foreach(fun (I) ->
            lists:foreach(fun (J) -> check(I, J, N - I - J, Y) end, lists:seq(0, N - I)) end,
            lists:seq(0, N)),
        {-1, -1, -1}
    catch
        throw:E -> E
    end,
    io:format("~B ~B ~B~n", [A, B, C]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
