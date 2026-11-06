"""A number-guessing game. Try changing the highest number below."""
import random

HIGHEST = 20


def main():
    secret = random.randint(1, HIGHEST)
    guesses = 0
    print(f"I'm thinking of a number from 1 to {HIGHEST}.")
    while True:
        answer = input("Your guess: ")
        if not answer.isdigit():
            print("Please type a whole number.")
            continue
        guess = int(answer)
        guesses += 1
        if guess < secret:
            print("Too low.")
        elif guess > secret:
            print("Too high.")
        else:
            print(f"You got it in {guesses} guesses!")
            return


if __name__ == "__main__":
    main()
