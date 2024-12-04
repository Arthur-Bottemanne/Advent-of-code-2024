<?php

$input = file("./input");

$inputArrayLeft = [];

$inputArrayRight = [];

foreach ($input as $inputLine) {
    $inputLineArray = explode("   ", $inputLine);

    $inputArrayLeft[] = $inputLineArray[0];
    $inputArrayRight[] = $inputLineArray[1];
}

sort($inputArrayLeft);
sort($inputArrayRight);

$sortedInput = [];

$arrayLength = count($input);

for ($count = 0; $count < $arrayLength; $count++) {
    if (isset($inputArrayLeft[$count]) && isset($inputArrayRight[$count])) {
        $sortedInput[] = [$inputArrayLeft[$count], $inputArrayRight[$count]];
    }
}

$result = 0;

foreach ($sortedInput as $sortedInputLine) {
    $result += abs($sortedInputLine[0] - $sortedInputLine[1]);
}

error_log($result);
