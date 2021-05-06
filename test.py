import random
import os
import time

def main():
    r = 100
    secret_number = random.randint(1, r + 1)
    x = 0
    guess = r//2
    flag = r//2
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
           # os.system("clear")
            print("Correct answer on the ", x, "try!")
            print("The secret number is", secret_number)
            break
main()