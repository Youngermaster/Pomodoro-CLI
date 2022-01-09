# Pomodoro-CLI

This is a pomodoro CLI to be more productive using the Pomodoro method.

## How to run

### Python option:

```shell
python3 main.py
```

Or

```powershell
python3 main.py
```

### Binary option:

```
pyinstaller --onefile main.py -n pomodoro-cli
cd dist
./pomodoro-cli
```

## How to build for your OS

- Make sure to install `Python 3`, then run the following commands:

### Linux

```shell
python3 -m venv pomodoro-env
source pomodoro-env/bin/activate
pip3 install -r requirements.txt
chmod +x ./linux_installation.sh
sudo ./linux_installation.sh
```

Then use wherever you want:

```
pomodoro-cli
```

### Windows

```powershell
python -m venv pomodoro-env
pomodoro-env\Scripts\activate.bat
pip install pyinstaller
pyinstaller --onefile main.py -n pomodoro-cli
```

After that place these three files wherever you want:

- `Assets` folder, where the sound effect is.
- `dist/pomodoro-cli` file, where the binary file is.
- `config.ini` file, where the config files for the path is, **You should mofify this file**.
