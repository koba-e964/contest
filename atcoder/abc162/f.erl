-module('Main').
-export([main/1]).

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(lists:duplicate(N, "~d"))),
    Rest = case N rem 2 of
        0 -> 1;
        1 -> 2
    end,
    ABList = [{A, B} || A <- lists:seq(0, Rest), B <- lists:seq(0, A)],
    Inf = 1000 * 1000 * 1000 * 1000 * 1000,
    Arr0 = maps:from_list([{{I, J}, -Inf} || I <- lists:seq(-1, N), J <- lists:seq(0, Rest)]),
    Arr1 = lists:foldl(fun ({I, Ai}, Acc) ->
        lists:foldl(fun ({J, K}, Acc) ->
            case I - K - 2 >= -1 of
                true ->
                    Value = maps:get({I - K - 2, J - K}, Acc),
                    case Value < -Inf div 2 of
                        true -> Acc;
                        false ->
                            Old = maps:get({I, J}, Acc),
                            Acc#{{I, J} => max(Value + Ai, Old)}
                    end;
                false -> Acc
            end
        end,
        Acc,
        ABList)
    end, Arr0#{{-1, 0} => 0}, lists:zip(lists:seq(1, N), A)),
    Ans = lists:max([maps:get({N - I, Rest - I}, Arr1) || I <- lists:seq(0, Rest)]),
    io:format("~B~n", [Ans]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
