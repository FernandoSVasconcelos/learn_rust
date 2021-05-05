import random
import os
def main():
    secret_number = random.randint(1, 101)
    x = 0
    while True:
        print("Guess the number!")
        x = x + 1
        guess = int(input("Please input your guess:"))
        if guess > secret_number:
            os.system("clear")
            print("Too big!")
        elif guess < secret_number:
            os.system("clear")
            print("Too small!")
        elif guess == secret_number:
            print("Correct answer on the ", x, "try!")
            print("The secret number is", secret_number)
            break
main()