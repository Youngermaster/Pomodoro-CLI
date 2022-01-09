# Pomodoro-CLI

This is a pomodoro CLI to be more productive.

## How to build for your OS

- Make sure to install `Python 3`, then run the following commands:

### Windows

```powershell
python -m venv pomodoro-env
pomodoro-env\Scripts\activate.bat
pip install pyinstaller
```

### Linux

```shell
python3 -m venv pomodoro-env
source pomodoro-env/bin/activate
pip3 install -r requirements.txt
chmod +x ./linux_installation.sh
sudo ./linux_installation.sh
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