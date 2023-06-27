#File: Tools_for_Success.py

#Program #1
#Birthday Calculator

#This program will calculate the age of the user 
#based on the year they were born

#Welcome Message
print("Welcome to the Birthday Calculator")
print("This program will calculate your age based on your year of birth")

#Get the current year
from datetime import date 
current_year = date.today().year  

#Get the year the user was born
birth_year = int(input("What year were you born in?"))

#Calculate the age of the user
age = current_year - birth_year

#Display result
print("You are "+str(age)+" years old")

#Program #2
#Rock, Paper, Scissors

#This program will allow the user to play the game of Rock, Paper, Scissors against the computer

#Welcome Message
print("Welcome to the game of Rock, Paper, Scissors!")

#Computer will assign user a choice of Rock, Paper, or Scissors
from random import randint
choices = ["rock","paper","scissors"]
computer_choice = choices[randint(0,2)]

#User will choose Rock, Paper, or Scissors
user_choice = input("Choose rock, paper, or scissors: ")

#Compare user choice and computer choice
if computer_choice == user_choice:
  print("The computer chose " + computer_choice + ". It's a tie!")
elif computer_choice == "rock" and user_choice == "paper":
  print("The computer chose rock. You win!")
elif computer_choice == "rock" and user_choice == "scissors":
  print("The computer chose rock. You lose!")
elif computer_choice == "paper" and user_choice == "rock":
  print("The computer chose paper. You lose!")
elif computer_choice == "paper" and user_choice == "scissors":
  print("The computer chose paper. You win!")
elif computer_choice == "scissors" and user_choice == "rock":
  print("The computer chose scissors. You win!")
else:
  print("The computer chose scissors. You lose!")

#Program #3
#Word Counter

#This program will count the number of words in a given string

#Welcome Message
print("Welcome to the Word Counter!")
print("This program will count the number of words in a given string")

#Get input from user
user_string = input("Enter your string: ")

#Split the string into separate words
words = user_string.split()

#Count the number of words
word_count = len(words)

#Display result
print("The number of words is "+str(word_count))

#Program #4
#Fizz Buzz

#This program will play the game of Fizz Buzz

#Welcome Message
print("Welcome to the game of Fizz Buzz!")

#Start loop from 1 to 100
for i in range(1, 101):
  #Multiple of 3 & 5
  if i % 3 == 0 and i % 5 == 0:
    print("Fizz Buzz")
  #Multiple of 3    
  elif i % 3 == 0:
    print("Fizz")
  #Multiple of 5
  elif i % 5 == 0:
    print("Buzz")
  #Otherwise
  else:
    print(i)

#Program #5
#Pig Latin Translator

#This program will translate a phrase into pig latin

#Welcome Message
print("Welcome to the Pig Latin Translator!")

#Get user input
phrase = input("Enter a phrase to translate: ")

#Split the phrase into separate words
words = phrase.split()

#Translate each word
pig_words = []
for word in words:
  if word[0] in "aeiou":
    pig_word = word+"way"
  else:
    pig_word = word[1:]+word[0]+"ay"
  pig_words.append(pig_word)
  
#Join the words back into a phrase
pig_phrase = " ".join(pig_words)

#Display pig latin phrase
print(pig_phrase) 

#Program #6
#Word Jumble

#This program will jumble up a given word

#Welcome Message
print("Welcome to the Word Jumble!")

#Get word input from user
word = input("What word do you want to jumble? ")

#Jumble the word
jumbled = "".join(random.sample(word,len(word)))

#Display jumbled word
print("Your jumbled word is "+jumbled)

#Program #7
#Factorial Calculator

#This program will calculate the factorial of a given number

#Welcome Message
print("Welcome to the Factorial Calculator!")

#Get number input from user
num = int(input("What number do you want to calculate the factorial of? "))

#Calculate the factorial
factorial = 1
for i in range(1,num+1):
  factorial = factorial * i

#Display result
print(str(num)+"! = "+str(factorial))

#Program #8
#BMI Calculator

#This program will calculate the BMI (body mass index) of the user

#Welcome Message
print("Welcome to the BMI Calculator!")

#Get height and weight from user
height = float(input("What is your height (in meters)? "))
weight = float(input("What is your weight (in kilograms)? "))

#Calculate BMI
bmi = weight / (height**2)

#Display result
print("Your BMI is "+str(bmi))

#Program #9
#Prime Number Checker

#This program will check to see if a given number is prime or not

#Welcome Message
print("Welcome to the Prime Number Checker!")

#Get number input from user
num = int(input("What number do you want to check? "))

#Check if number is prime
is_prime = True
for i in range(2, num):
  if num % i == 0:
    is_prime = False
    break
  
#Display result
if is_prime:
  print(str(num)+" is a prime number")
else:
  print(str(num)+" is not a prime number")

#Program #10
#Guessing Game

#This program will generate a random number for the user to guess

#Welcome Message
print("Welcome to the Guessing Game!")

#Generate random number
from random import randint
number = randint(1,100)

#Get user input
guess = int(input("Guess a number between 1 and 100: "))

#Loop until user guesses the correct number
while guess != number:
  if guess < number:
    print("Too Low!")
    guess = int(input("Guess again: "))
  else:
    print("Too High!")
    guess = int(input("Guess again: "))

#Display Success Message
print("You guessed it! The number was "+str(number))