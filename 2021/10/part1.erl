-module(part1).
-export([main/1]).

main([]) ->
  Input = read_input(),
  Scores = [score(Line) || Line <- Input],
  Answer = lists:sum(Scores),
  io:format("Answer: ~w\n", [Answer]).


read_input() ->
  {ok, Content} = file:read_file("input.txt"),
  List = binary:split(Content, <<"\n">>, [trim, global]),
  [[list_to_atom([Char]) || Char <- binary:bin_to_list(Item)] || Item <- List].


score(Line) ->
  case score_line([], Line) of
    valid -> 0;
    {invalid, Score} -> Score
  end.

% Opening new ones is always allowed.
score_line(Open, ['(' | Input]) -> score_line(['(' | Open], Input);
score_line(Open, ['[' | Input]) -> score_line(['[' | Open], Input);
score_line(Open, ['{' | Input]) -> score_line(['{' | Open], Input);
score_line(Open, ['<' | Input]) -> score_line(['<' | Open], Input);

% Closing can be done when the matching pair is there.
score_line(['(' | Open], [')' | Input]) -> score_line(Open, Input);
score_line(['[' | Open], [']' | Input]) -> score_line(Open, Input);
score_line(['{' | Open], ['}' | Input]) -> score_line(Open, Input);
score_line(['<' | Open], ['>' | Input]) -> score_line(Open, Input);

% Closing without that matching pair is invalid.
score_line(_, [')' | _]) -> {invalid, 3};
score_line(_, [']' | _]) -> {invalid, 57};
score_line(_, ['}' | _]) -> {invalid, 1197};
score_line(_, ['>' | _]) -> {invalid, 25137};

% When there is no input left, it was valid.
score_line(_, []) -> valid.
