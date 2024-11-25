-module(aoc).
-import(lists,[foldl/3,map/2,sum/1, sort/1, sort/2]).
-export([start/0, do_work/0]).

start() -> 
  Input = map(fun split_input/1, readlines("input.txt")),
  Part = os:getenv("part", "part1"),
  Worker = spawn(aoc, do_work,[]),
  if 
    Part == "part2" ->
      Worker ! {part2, Input};
    true -> 
      Worker ! {part1, Input} 
  end,
  io:fwrite("~p~n", ["Done"]).

do_work() ->
  receive
    {part1, Input} -> 
      Sorted = sort(fun sort_hand/2, Input),
      Tot = total_winnings(Sorted, 1, 0),
      io:fwrite("~p~n", [Tot]);
    {part2, Input} -> 
      Sorted = sort(fun sort_hand_j/2, Input),
      Tot = total_winnings(Sorted, 1, 0),
      io:fwrite("~p~n", [Tot])
  end.

total_winnings([], _, Acc) -> Acc;
total_winnings([{_, B}| X_], R, Acc) -> total_winnings(X_, R+1, Acc + B * R).

sort_hand({L, _}, {R, _}) ->
  LType = type(sort(L)),
  RType = type(sort(R)),
  Order = order_hands(L, R),
  if 
    LType < RType -> true;
    LType > RType -> false;
    true -> Order 
  end.

order_hands([L | L_], [R | R_]) ->
  LOrder = order(L),
  ROrder = order(R),
  if
    LOrder == ROrder -> order_hands(L_, R_);
    LOrder > ROrder -> false;
    true -> true
  end.

order($A) -> 12;
order($K) -> 11;
order($Q) -> 10;
order($J) -> 9;
order($T) -> 8;
order($9) -> 7;
order($8) -> 6;
order($7) -> 5;
order($6) -> 4;
order($5) -> 3;
order($4) -> 2;
order($3) -> 1;
order($2) -> 0.

type([X,X,X,X,X]) -> 7;
type([X,X,X,X,_]) -> 6;
type([_,X,X,X,X]) -> 6;
type([X,X,X,Y,Y]) -> 5;
type([Y,Y,X,X,X]) -> 5;
type([_,_,X,X,X]) -> 4;
type([X,X,X,_,_]) -> 4;
type([_,X,X,X,_]) -> 4;
type([X,X,Y,Y,_]) -> 3;
type([_,X,X,Y,Y]) -> 3;
type([X,X,_,Y,Y]) -> 3;
type([X,X,_,_,_]) -> 2;
type([_,X,X,_,_]) -> 2;
type([_,_,X,X,_]) -> 2;
type([_,_,_,X,X]) -> 2;
type(_) -> 1.

sort_hand_j({L, _}, {R, _}) ->
  LType = type_j(sort(L)),
  RType = type_j(sort(R)),
  Order = order_hands_j(L, R),
  if 
    LType < RType -> true;
    LType > RType -> false;
    true -> Order 
  end.

order_hands_j([L | L_], [R | R_]) ->
  LOrder = order_j(L),
  ROrder = order_j(R),
  if
    LOrder == ROrder -> order_hands_j(L_, R_);
    LOrder > ROrder -> false;
    true -> true
  end.

order_j($A) -> 12;
order_j($K) -> 11;
order_j($Q) -> 10;
order_j($T) -> 9;
order_j($9) -> 8;
order_j($8) -> 7;
order_j($7) -> 6;
order_j($6) -> 5;
order_j($5) -> 4;
order_j($4) -> 3;
order_j($3) -> 2;
order_j($2) -> 1;
order_j($J) -> 0.

type_j(Hand) ->
  CardCounts = sort(fun sort_occ/2, count_cards(Hand)),
  M = max(CardCounts),
  R = replace(M, Hand),
  type(sort(R)).

sort_occ({_, X}, {_, Y}) ->
  X >= Y.

replace(C, Hand) -> replace(C, Hand, []).
replace(_, [], NewHand) -> NewHand;
replace(C, [$J|Rest], NewHand) -> replace(C, Rest, NewHand++[C]);
replace(C, [X|Rest], NewHand) -> replace(C, Rest, NewHand++[X]).

max([{$J, _}, {X,_}| _]) -> X;
max([{X,_}| _]) -> X.

count_cards(Hand) -> count_cards(sets:to_list(sets:from_list(Hand)), Hand, []).
count_cards([], _, Occ) -> 
  Occ;
count_cards([Card | Rest], Hand, Occ) -> 
  count_cards(Rest, Hand, Occ ++ [{Card, count(Card, Hand)}]).

count(Card, Hand) -> count(Card, Hand, 0).
count(_, [], Count) -> Count;
count(X, [X|Rest], Count) -> count(X, Rest, Count+1);
count(X, [_|Rest], Count) -> count(X, Rest, Count).


readlines(Filename) ->
    {ok, Blob} = file:read_file(Filename),
    Content = erlang:binary_to_list(Blob),
    map(fun erlang:binary_to_list/1, re:split(Content, "\n")).


split_input(Row) ->
  Parts = string:split(Row, " "),
  {lists:nth(1, Parts), erlang:list_to_integer(lists:nth(2, Parts))}.