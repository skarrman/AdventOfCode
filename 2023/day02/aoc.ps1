$inputContent = Get-Content ./input.txt
$part = $ENV:part

$id_regex = "Game\s(?<id>\d+):"
$red_regex = "(\d+)\sred"
$blue_regex = "(\d+)\sblue"
$green_regex = "(\d+)\sgreen"

if ($part -eq "part2") {
    $result = 0 
    $__ = $inputContent | foreach {
        $_ -match $id_regex
        $id = $matches["id"]
        $red=0; [Regex]::Matches($_,$red_regex) |  foreach { if($red -lt [int]$_.Groups[1].value ) {$red = [int]$_.Groups[1].value }};
        $green=0; [Regex]::Matches($_,$green_regex) |  foreach { if($green -lt [int]$_.Groups[1].value ) {$green = [int]$_.Groups[1].value }};
        $blue=0; [Regex]::Matches($_,$blue_regex) |  foreach { if($blue -lt [int]$_.Groups[1].value ) {$blue = [int]$_.Groups[1].value }};
        $power=$red*$green*$blue
        $result += $power
    }
    Write-Host $result
} else {
    $result = 0 
    $__ = $inputContent | foreach {
        $_ -match $id_regex
        $id = $matches["id"]
        $red=0; [Regex]::Matches($_,$red_regex) |  foreach { if($red -lt [int]$_.Groups[1].value ) {$red = [int]$_.Groups[1].value }};
        $green=0; [Regex]::Matches($_,$green_regex) |  foreach { if($green -lt [int]$_.Groups[1].value ) {$green = [int]$_.Groups[1].value }};
        $blue=0; [Regex]::Matches($_,$blue_regex) |  foreach { if($blue -lt [int]$_.Groups[1].value ) {$blue = [int]$_.Groups[1].value }};
        
        if(($red -le 12) -and  ($green -le 13) -and ($blue -le 14)){
            $result += $id
        }
    }
    Write-Host $result
}