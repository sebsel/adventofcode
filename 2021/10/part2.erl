-module(part2).
-export([main/1]).

main([]) ->
  Input = read_input(),
  Scores = [score(Line) || Line <- Input],
  SortedValidScores = lists:sort([Score || {valid, Score} <- Scores]),
  Middle = (length(SortedValidScores) div 2) + 1,
  Answer = lists:nth(Middle, SortedValidScores),
  io:format("Answer: ~w\n", [Answer]).

read_input() ->
  {ok, Content} = file:read_file("input.txt"),
  List = binary:split(Content, <<"\n">>, [trim, global]),
  [[list_to_atom([Char]) || Char <- binary:bin_to_list(Item)] || Item <- List].


score(Line) -> score_line([], Line, 0).

% Opening new ones is always allowed.
score_line(Open, ['(' | Input], Score) -> score_line(['(' | Open], Input, Score);
score_line(Open, ['[' | Input], Score) -> score_line(['[' | Open], Input, Score);
score_line(Open, ['{' | Input], Score) -> score_line(['{' | Open], Input, Score);
score_line(Open, ['<' | Input], Score) -> score_line(['<' | Open], Input, Score);

% Closing can be done when the matching pair is there.
score_line(['(' | Open], [')' | Input], Score) -> score_line(Open, Input, Score);
score_line(['[' | Open], [']' | Input], Score) -> score_line(Open, Input, Score);
score_line(['{' | Open], ['}' | Input], Score) -> score_line(Open, Input, Score);
score_line(['<' | Open], ['>' | Input], Score) -> score_line(Open, Input, Score);

% Closing without that matching pair is invalid.
score_line(_, [')' | _], 0) -> {invalid, 3};
score_line(_, [']' | _], 0) -> {invalid, 57};
score_line(_, ['}' | _], 0) -> {invalid, 1197};
score_line(_, ['>' | _], 0) -> {invalid, 25137};

% Score the pairs that are still open
score_line(['(' | Open], [], Score) -> score_line(Open, [], Score * 5 + 1);
score_line(['[' | Open], [], Score) -> score_line(Open, [], Score * 5 + 2);
score_line(['{' | Open], [], Score) -> score_line(Open, [], Score * 5 + 3);
score_line(['<' | Open], [], Score) -> score_line(Open, [], Score * 5 + 4);
score_line([], [], Score) -> {valid, Score}.
