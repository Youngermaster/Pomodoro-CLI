pyinstaller --onefile main.py -n pomodoro-cli
sudo mkdir -p /usr/share/pomodoro-cli
sudo cp -r Assets dist/pomodoro-cli config.ini /usr/share/pomodoro-cli
sudo ln -s /usr/share/pomodoro-cli/pomodoro-cli /usr/bin/pomodoro-cli