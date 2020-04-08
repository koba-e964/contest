-module('Main').
-export([main/1]).

gcd(A, 0) -> A;
gcd(A, B) -> gcd(B, A rem B).

read_ints(N) ->
	Pat = lists:flatten(["~d" || _ <- lists:seq(1, N)]),
	input(Pat).

solve() ->
	[N, Q] = input("~d ~d"),
	A = lists:zip(read_ints(N), lists:seq(1, N)),
	Qs = read_ints(Q),
	Acc = lists:reverse(lists:foldl(fun ({X, Idx2}, L = [{B, _Idx} | _Rest]) ->
		G = gcd(X, B),
		case G =:= B of
			true -> L;
			false -> [{G, Idx2} | L]
		end
	end, [{0, 0}], A)),
    Pat = lists:flatten(["~B~n" || _I <- lists:seq(1, Q)]),
	Ans = lists:map(fun (Qv) ->
		{_, Ans} = lists:foldl(fun ({V, Idx}, Acc0) ->
			case Acc0 of
				{indet, _} ->
					case gcd(V, Qv) of
						1 -> {det, Idx};
						Gcd -> {indet, Gcd}
					end;
				_ -> Acc0
			end
		end,
		{indet, 0},
		Acc),
		Ans
	end, Qs),
	io:format(Pat, Ans),
	ok.

main(_) ->
	solve(),
	halt().

input(Pat) ->
	{ok, L} = io:fread("", Pat),
	L.