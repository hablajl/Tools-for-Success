#!/usr/bin/perl

# This program demonstrates basic tools for success in Perl 

# Include necessary modules

use strict;
use warnings;
use CGI qw/:standard/;

# Declare variables

my ( $firstName, $lastName, $fullName, $age, $birthMonth );

# Prompt user for input

print "What is your first name? ";
$firstName = <STDIN>;
chomp($firstName);

print "What is your last name? ";
$lastName = <STDIN>;
chomp($lastName);

print "What is your age? ";
$age = <STDIN>;
chomp($age);

print "What is your birth month? ";
$birthMonth = <STDIN>;
chomp($birthMonth);

# Assign values to variables

$fullName = $firstName . " " . $lastName;

# Display results

print "Thank you for providing the following information:\n";
print "Name: $fullName\n";
print "Age: $age\n";
print "Birth Month: $birthMonth\n";

# Use conditionals to provide user feedback

if ($birthMonth eq "August") {
print "Happy Birthday!\n";
}
elsif ($age < 18) {
print "You are too young for this application.\n";
}
else {
print "Thank you for applying.\n";
}

# Read from a file

open(my $data, "<", "data.txt") or die "Could not open data.txt: $!";

my @dataArray = <$data>;
 
close($data);

# Write to a file

open(my $data, "<", "data.txt") or die "Could not open data.txt: $!";
 
print $data @dataArray;
 
close($data);

# Create an HTML response

print header;
print start_html("Tools for Success");
print h1("Tools for Success");
print p("This program demonstrates basic tools for success in Perl.");
print end_html;