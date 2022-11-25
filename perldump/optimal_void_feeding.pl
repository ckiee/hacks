#!/usr/bin/env perl

sub draw_bar {
    my ($file, $title, $width, $hash) = @_;
    open my $GNUPLOT, '|-', 'gnuplot' or die $!;
    print {$GNUPLOT} << "__GNUPLOT__";
    set term pngcairo size 1800, 600
    set output "$file"
    set title "$title"
    set yrange [0:]
    set logscale x
    set xlabel "dd bs count"
    set ylabel "pipe speed"
    plot '-' using 1:2:1 with boxes fs solid notitle
__GNUPLOT__

    for my $x (keys %$hash) {
        say {$GNUPLOT} "$x\t$hash->{$x}";
    }
    close $GNUPLOT;
}

my %plot;
print STDERR "bs,mb$/";for my$exp(1..13) {
    $bs=2**$exp;
    $dbs=$bs."M";
    $_=`timeout -k 0.1s -s SIGUSR1 1s dd if=/dev/zero of=/dev/null bs=$dbs 2>&1`;
    /(\d+(?:\.\d+)?) ([GM]B)\/s/;
    $b=$1;
    if ($2 =~ /GB/) {
        $b *= 1024;
    }
    print "$bs = $b$/";
    $plot{$bs}=$b;
}
draw_bar("/tmp/void.png", "optimal void feeding",1, \%plot)
