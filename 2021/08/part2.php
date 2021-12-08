<?php

$f = fopen('input.txt', 'r');

$total = 0;
while ($line = fgets($f)) {
    $split = explode('|', $line);
    $numbers = explode(' ', trim($split[0]));
    $numbers = array_map('to_set', $numbers);
    $dict = [];

    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 2) {
        $dict[1] = $set; unset($numbers[$i]); break;
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 3) {
        $dict[7] = $set; unset($numbers[$i]); break;
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 4) {
        $dict[4] = $set; unset($numbers[$i]); break;
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 7) {
        $dict[8] = $set; unset($numbers[$i]); break;
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 5) {
        if (($set | $dict[7]) === $set) {
            $dict[3] = $set; unset($numbers[$i]); break;
        }
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 6) {
        if (($set | $dict[4]) === $set) {
            $dict[9] = $set; unset($numbers[$i]); break;
        }
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 6) {
        if (($set | $dict[1]) !== $set) {
            $dict[6] = $set; unset($numbers[$i]); break;
        }
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 6) {
        $dict[0] = $set; unset($numbers[$i]); break;
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 5) {
        if (($set | $dict[9]) === $dict[9]) {
            $dict[5] = $set; unset($numbers[$i]); break;
        }
    }
    foreach ($numbers as $i => $set) if (substr_count(decbin($set), '1') == 5) {
        $dict[2] = $set; unset($numbers[$i]); break;
    }

    ksort($dict);
    $dict = array_flip($dict);
    foreach ($dict as $set => $num)
        echo "$num ".str_pad(decbin($set), 7, '0', STR_PAD_LEFT)."\n";

    // Lookup
    $numbers = explode(' ', trim($split[1]));
    $numbers = array_map('to_set', $numbers);
    $number = (
        $dict[$numbers[0]] * 1000
        + $dict[$numbers[1]] * 100
        + $dict[$numbers[2]] * 10
        + $dict[$numbers[3]]
    );
    echo $number."\n";
    $total += $number;
}

echo "Answer: $total\n";

fclose($f);

function to_set($string)
{
    $set = 0;
    foreach ([
        'a' => 1, 'b' => 2, 'c' => 4, 'd' => 8,
        'e' => 16, 'f' => 32, 'g' => 64,
    ] as $char => $pos) {
        if (strpos($string, $char) !== false) $set |= $pos;
    }
    return $set;
}
