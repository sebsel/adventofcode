<?php

$lines = explode("\n", trim(file_get_contents('input.txt', 'r')));
const LINE_LEN = 12;

# $lines = [
#     '00100', '11110', '10110', '10111', '10101', '01111',
#     '00111', '11100', '10000', '11001', '00010', '01010',
# ];
# const LINE_LEN = 5;

function mostCommon(array $lines, int $position): int
{
    $count = [0, 0];
    foreach ($lines as $line) {
        $count[$line[$position]]++;
    }
    return $count[0] > $count[1] ? 0 : 1;
}

function findRating(array $lines, callable $compare): string
{
    for ($i = 0; $i < LINE_LEN; $i++) {
        $common = mostCommon($lines, $i);
        foreach ($lines as $j => $line) {
            if ($compare($line[$i], $common)) {
                unset($lines[$j]);
            }
        }

        if (count($lines) === 1) {
            return array_pop($lines);
        }
    }

    throw new Error('Not found');
}

$oxigen = findRating($lines, fn($a, $b) => $a != $b);
$co2 = findRating($lines, fn($a, $b) => $a == $b);

echo "Oxigen: $oxigen\n",
    "CO2: $co2\n";

$oxigen = bindec($oxigen);
$co2 = bindec($co2);
$answer = $oxigen * $co2;

echo "Answer $answer\n";
