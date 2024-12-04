<?php

$input = file("./input");
$input = array_map('trim', $input);

$inputArrayLeft = [];

$inputArrayRight = [];

foreach ($input as $inputLine) {
    $inputLineArray = explode("   ", $inputLine);

    $inputArrayLeft[] = $inputLineArray[0];
    $inputArrayRight[] = $inputLineArray[1];
}

error_log(print_r($inputArrayLeft, true));
error_log(print_r($inputArrayRight, true));

$arrayLength = count($inputArrayLeft);

$inputArrayRightCounts = array_count_values($inputArrayRight);

error_log(print_r($inputArrayRightCounts));

error_log("===========");

$result = 0;

for ($count = 0; $count < $arrayLength; $count++) {
    $element = $inputArrayLeft[$count];

    $multiplier = $inputArrayRightCounts[$element] ?? 0;

    error_log($element);
    error_log($multiplier);
    error_log($element * $multiplier);
    error_log("===========");

    $result += $element * $multiplier;
}


error_log($result);
