# Pomodoro-CLI

This is a pomodoro CLI to be more productive.

## Requirementes

Install `Python 3`, and then install the `playsound` library:

```
pip install playsound
```

or

```
pip3 install playsound
```

## How to build

- Make sure to install `pyinstaller`:

```
pip install pyinstaller
```

or 

```
pip3 install pyinstaller
```

- Then use:

```
pyinstaller --onefile main.py -n pomodoro-cli
```

## How to run


- Python option:

```
python3 main.py
```

- Binary option:

```
pyinstaller --onefile main.py -n pomodoro-cli
cd dist
./pomodoro-cli
```