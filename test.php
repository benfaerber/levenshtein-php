<?php

$l = new Levenshtein(); 
var_dump($l->hello_world());

$a = "apple";
$b = "arrle";
var_dump($l->distance($a, $b));

$a = file_get_contents("./test_data/expected_donkey_0.zpl.txt");
$b = file_get_contents("./test_data/expected_donkey_1.zpl.txt");
var_dump($l->distance($a, $b));
