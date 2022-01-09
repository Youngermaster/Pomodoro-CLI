import configparser
import os
import sys
import time
from playsound import playsound

config = configparser.ConfigParser()

# Default values
tone_path = f'{os.getcwd}/Assets/Audio/OnePieceGoldBellSoundEffect.mp3'
work_timer_minutes = 25
rest_timer_minutes = 5


def get_config() -> str:
    try:
        config.read('config.ini')
        tone_path = config['INSTALLATION']['PATH']
    except:
        tone_path = f'{os.getcwd}/Assets/Audio/OnePieceGoldBellSoundEffect.mp3'
    return tone_path


def timer_countdown(minutes, type):
    minutes_in_seconds = minutes * 60
    for remaining in range(minutes_in_seconds, 0, -1):
        sys.stdout.write("\r")
        sys.stdout.write("{:2d} seconds remaining.".format(remaining))
        sys.stdout.flush()
        time.sleep(1)

    if type == "work":
        sys.stdout.write(f"\rGo to rest for {rest_timer_minutes} minutes\n")
    elif type == "rest":
        sys.stdout.write(
            f"\rLet's get back to work! For another {work_timer_minutes} minutes\n")
    playsound(tone_path)


def print_menu():
    print(
        """
        |================================================================|
        |                         CLI Pomodoro                           |
        |================================================================|
        |                                                                |
        | Select an option:                                              |
        |                                                                |
        | 1. Start pomodoro                                              |
        | 2. Change time values                                          |
        | 3. Quit                                                        |
        | 4. LICENSE                                                     |
        |                                                                |
        |================================================================|

        """
    )


def print_license():
    print(
        """
        MIT License

        Copyright (c) 2021 Juan Manuel Young Hoyos

        Permission is hereby granted, free of charge, to any person obtaining a copy
        of this software and associated documentation files (the "Software"), to deal
        in the Software without restriction, including without limitation the rights
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        copies of the Software, and to permit persons to whom the Software is
        furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all
        copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        SOFTWARE.
        """
    )


if __name__ == "__main__":
    tone_path = get_config()
    while True:
        print_menu()
        user_option = int(input())

        if user_option == 1:
            timer_countdown(work_timer_minutes, "work")
            timer_countdown(rest_timer_minutes, "rest")

        elif user_option == 2:
            new_work_minutes = int(
                input("Please set the desired minutes to work: "))
            new_rest_minutes = int(
                input("Please set the desired minutes to rest: "))

            work_timer_minutes = new_work_minutes
            rest_timer_minutes = new_rest_minutes

        elif user_option == 3:
            break

        elif user_option == 4:
            print_license()

        else:
            print("Please the previously defined options, please")

        sys.stdout.flush()
