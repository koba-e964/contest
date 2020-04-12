-module('Main').
-export([main/1]).

calc2(N, I, Ai, Arr) ->
    length(lists:filter(fun (J) ->
        Aj = maps:get(I - J, Arr),
        Ak = maps:get(I + J, Arr),
        Ai =/= Aj andalso Ai =/= Ak andalso Aj =/= Ak
    end, lists:seq(1, min(I - 1, N - I)))).

calc(N, Arr) ->
    lists:sum([calc2(N, I, maps:get(I, Arr), Arr) || I <- lists:seq(1, N)]).

solve() ->
    [N, S] = input("~d~s"),
    Arr = maps:from_list(lists:zip(lists:seq(1, N), S)),
    [R, G, B] = [length(lists:filter(fun (X) -> X =:= Chara end, S)) || Chara <- "RGB"],
    io:format("~B~n", [R * G * B - calc(N, Arr)]),
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
