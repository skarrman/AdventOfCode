open(my $in,  "<",  "input.txt") or die;
%all_numbers = ("1", "2", "3", "4", "5", "6", "7", "8", "9", "0");
%numbers_map = ();
%symbols = ();
@numbers = ();
$key = 0;
$number = "";
$y = 0;
while (<$in>) {
    my $x = 0;
    my @chars = split(//, $_);
    foreach (@chars){
        $char = $_ eq "." ? "," : $_; 
        $char = $char eq "*" ? "gear" : $char; 
        if ($char eq "\n") {
            if($number ne ""){
                push(@numbers, $number);
                $key++;
                $number = "";
            }
            next;
        }
        if (!grep(/^$char$/, %all_numbers) && $number ne "") {
            push(@numbers, $number);
            $key++;
            $number = "";
        } elsif (grep(/^$char$/, %all_numbers)) {
            $number = "$number$char";
            $numbers_map{"$x,$y"} = $key;
        } 
        if ($char ne "," && !grep(/^$char$/, %all_numbers)) {
            $symbols{"$x,$y"}=$char;
        }
        $x++;   
    }
    $y++;
}

my @ds = (-1, 0, 1);
my %adjacent_keys = ();
my $gear_ratio=0;
foreach (keys %symbols){
    @parts = split(/,/, $_);
    my $x = @parts[0];
    my $y = @parts[1];
    my %adj_nums = ();
    foreach my $dy (@ds) {
        foreach my $dx (@ds) {
            if($dx == 0 && $dy == 0){
                next;
            }
            my $x_ = $x+$dx;
            my $y_ = $y+$dy;
            my $key = "$x_,$y_";
            if(exists $numbers_map{$key}){
                my $k = $numbers_map{$key};
                $adj_nums{$k}=1;
                $adjacent_keys{$k} = 1;
            }else{
                }
        }
    }
    if($symbols{$_} eq "gear" && keys %adj_nums == 2){
        my $r = 1;
        foreach (keys %adj_nums){
            $r *= $numbers[$_];
        }
        $gear_ratio += $r;
    }
}

if($ENV{"part"} eq "part2"){
    print "$gear_ratio\n";
}else{
    $sum = 0;
    foreach (keys %adjacent_keys){
        $sum+=$numbers[$_];
    }
    print "$sum\n";
}