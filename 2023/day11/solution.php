<?php
$part = $_ENV["part"];
$inputfile = fopen("input.txt", "r") or die("Unable to open file!");
$input = fread($inputfile, filesize("input.txt"));
fclose($inputfile);


$galaxies = [];
$emptyrows = [];
$rows = explode("\n", $input);

$y = 0;
foreach ($rows as $row) {
    $cols = str_split($row);
    $x = 0;
    $empty = true;
    foreach ($cols as $col) {
        if ($col == '#') {
            $galaxies[] = [$x, $y];
            $empty = false;
        }
        $x++;
    }
    if ($empty)
        $emptyrows[] = $y;

    $y++;
}

$emptycols = [];
for ($x = 0; $x < strlen($rows[0]); $x++) {
    $empty = true;
    foreach ($galaxies as $g) {
        [$gx, $y] = $g;
        if ($x == $gx) {
            $empty = false;
            break;
        }
    }
    if ($empty)
        $emptycols[] = $x;
}

function emptybetween($empty, $i, $j, $dist)
{
    $s = 0;
    $max = max($i, $j);
    $min = min($i, $j);
    foreach ($empty as $c) {
        if ($min < $c && $c < $max)
            $s += $dist;
    }
    return $s;
}

$dist = $part == "part2" ? 1000000 - 1 : 1;

$lengths = [];
for ($gf = 0; $gf < count($galaxies) - 1; $gf++) {
    for ($gt = count($galaxies) - 1; $gt > $gf; $gt--) {
        [$fx, $fy] = $galaxies[$gf];
        [$tx, $ty] = $galaxies[$gt];
        $lengths[] = abs($fy - $ty) + emptybetween($emptyrows, $fy, $ty, $dist) + abs($fx - $tx) + emptybetween($emptycols, $fx, $tx, $dist);
    }
}
echo (array_sum($lengths));
?>