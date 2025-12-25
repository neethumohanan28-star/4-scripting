<?php
echo "Enter your name: ";
$name = trim(fgets(STDIN));

$current_time = date("Y-m-d H:i:s");

echo "Hello $name, right now the time is $current_time";
?>

