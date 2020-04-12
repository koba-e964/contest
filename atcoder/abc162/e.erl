-module('Main').
-export([main/1]).

-define(MOD, 1000000007).

pow(A, E) -> pow(A, E, 1).
pow(_A, 0, Acc) -> Acc;
pow(A, E, Acc) when E rem 2 =:= 1 -> pow(A * A rem ?MOD, E div 2, Acc * A rem ?MOD);
pow(A, E, Acc) -> pow(A * A rem ?MOD, E div 2, Acc rem ?MOD).

solve() ->
    [N, K] = input("~d~d"),
    Arr0 = maps:from_list([{I, pow(K div I, N)} || I <- lists:seq(1, K)]),
    Arr1 = lists:foldl(fun (I, Acc) ->
        lists:foldl(fun (J, Acc) ->
            Value = (maps:get(I, Acc) - maps:get(I * J, Acc) + ?MOD) rem ?MOD,
            Acc#{I => Value}
        end, Acc, lists:seq(2, K div I))
    end, Arr0, lists:reverse(lists:seq(1, K))),
    Sum = maps:fold(fun (I, Val, Acc) ->
        (Acc + I * Val) rem ?MOD
    end, 0, Arr1),
    io:format("~B~n", [Sum]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
