<?php

$f = fopen('input.txt', 'r');

$counts = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

while ($line = fgets($f)) {
    for ($i = 0; $i < 12; $i++) {
        $counts[$line[$i]][$i]++;
    }
}

$gamma = str_repeat('0', 12);
$epsilon = str_repeat('0', 12);
for ($i = 0; $i < 12; $i++) {
    echo "$i {$counts[0][$i]} {$counts[1][$i]} \n";
    if ($counts[0][$i] > $counts[1][$i]) {
        $gamma[$i] = '1';
    } else {
        $epsilon[$i] = '1';
    }
}

echo $gamma.' '.$epsilon."\n";

$gamma = bindec($gamma);
$epsilon = bindec($epsilon);
$answer = $gamma * $epsilon;

// Yep I needed debug statements this time.
echo "Answer $answer\n";
