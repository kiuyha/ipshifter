# IPShifter

IPShifter is a tool that can change ip address in specific interval using TOR. This is actually a remake from [Tornet](https://github.com/ByteBreach/tornet) but change to rust and fix some issues such as using `pkill -f tor` that cause computer to restart since it also kill other programs like `monitor`. Also make it open source and available on github.

## Benefits

- **Enhanced Privacy**: By regularly changing your IP address, IPShifter makes it much harder for websites and trackers to monitor your online activity.
- **Increased Security**: Frequent IP changes can help protect you from targeted attacks and make it more difficult for malicious actors to track your online presence.
- **Anonymity**: Using Tor, IPShifter helps you maintain a high level of anonymity while browsing the internet.
- **Ease of Use**: IPShifter is designed to be simple and easy to use, whether you prefer command-line tools or integrating it directly into your Python scripts.
- **Protection from Tracking**: With your IP address changing frequently, tracking services and advertisers will find it more challenging to build a profile on you.
- **Peace of Mind**: Knowing that your IP address is regularly changed can give you confidence in your online privacy and security.

## Installation

1. Apt
```bash
curl -fsSL https://kiuyha.my.id/ipshifter-apt/public.key | gpg --dearmor | sudo tee /usr/share/keyrings/ipshifter.gpg > /dev/null

echo "deb [signed-by=/usr/share/keyrings/ipshifter.gpg] https://kiuyha.my.id/ipshifter-apt stable main" | sudo tee /etc/apt/sources.list.d/ipshifter.list

sudo apt update
sudo apt install ipshifter
```

## Usage

IPShifter provides a command-line interface for easy use. Here are the available options:

```bash
ipshifter -i <seconds> -c <number>
```

- `-i` or `--interval` (optional): Time in seconds between IP changes (default is 10 seconds).
- `-c` or `--count` (optional): Number of times to change the IP (default is unlimited).
- `-d` or `--detach` (optional): Run IPShifter in the background.
- `-s` or `--stop` (optional): Run IPShifter in the background.
- `--help`: Show the help message and exit.
- `--version`: Show the version number and exit.

## How It Works

IPShifter uses the Tor network to route your internet traffic through multiple nodes, effectively masking your IP address. By periodically changing the IP address, IPShifter ensures that your online activity remains anonymous and secure. This can be particularly useful for:

- **Privacy enthusiasts** who want to minimize their digital footprint.
- **Security professionals** who need to conduct penetration testing or other security assessments without revealing their true IP address.
- **Journalists and activists** operating in regions with internet censorship or surveillance.

### Examples

Change the IP address every 30 seconds, for a total of 5 times:

```bash
ipshifter -i 30 -c 5
```

Change the IP address every 60 seconds indefinitely:

```bash
ipshifter -i 60
```

Run IPShifter in the background:

``` bash
ipshifter -d
```

Stop IPShifter running in the background:

```bash
ipshifter -s
```

## Configuration

1. **GNOME**. This for GNOME users and only applies to GUI Applications.:
    - Go to `Settings` > `Network` > `Proxy`.
    - Turn on `Network Proxy`.
    - Enter `127.0.0.1:9050` in `configuration ` .
    - Click `Save`.
<img src="https://raw.githubusercontent.com/kiuyha/ipshifter/main/assets/gnome.png" alt="GNOME Configuration Example" />


## Thanks

Thank you for using IPShifter! We hope this tool helps you secure your network and maintain your privacy. If you have any feedback or suggestions, please feel free to reach out to us.