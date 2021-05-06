import random
import os
import time

def main():
    secret_number = random.randint(1, 101)
    x = 0
    guess = 50
    flag = 50
    while True:
        time.sleep(0.5)
        print("Guess the number!")
        x = x + 1
        if guess > secret_number:
            print("Your guess is", guess)
            print("But it is too big!")
            flag = (flag//2)
            if flag >= 1.5 and flag < 2:
                flag = 2
            elif flag == 0:
                flag = 1
            else:
                flag = flag
            guess = guess - flag
        elif guess < secret_number:
            print("Your guess is", guess)
            print("But it is too small!")
            flag = (flag//2)
            if flag >= 1.5 and flag < 2:
                flag = 2
            elif flag == 0:
                flag = 1
            else:
                flag = flag
            guess = guess + flag
        elif guess == secret_number:
            os.system("clear")
            print("Correct answer on the ", x, "try!")
            print("The secret number is", secret_number)
            break
main()