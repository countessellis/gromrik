# AI Rig Setup and Optimization

Below is details on the AI rig I wrote Gromrik against, and optimizations I performed. This is both very small resource-wise compared to what is typically used for similar workloads. This is intentional.

## Minerva

I named my AI rig **minerva** after the Roman goddess, the rough equivalant of Athena, who I see as the Greek goddess of technology, in a modern sense. For home stuff, I usually name my machines and devices after figures from mythology, usually Greek, Roman, Etruscan, or Norse, choosing one appropriate for the role of the device. The name Minerva likely has its roots in *menos*, "thought", or *meh-nos*, "moon". Among other associations, Minerva was the goddess of wisdon, law, trade, and strategy. Seemed quite apropos for an AI rig.


## Hardware

The hardware I am using for my AI rig for this project is as follows:

* CPU: AMD Ryzen 7 8745HS (8 cores, 16 threads, Zen 4 architecture)
* GPU: Integrated AMD Radeon 780M (12 compute units, 24 AI accelerators, gfx1103 architecture)
  * Execution Backend: Operates via the Vulkan API backend (via Ollama/Mesa RADV) to ensure 100% stable GPU tensor compute offloading on Linux, bypassing the platform constraints of mobile ROCm.
* RAM: 16GB 5600 MT/s DDR5 SODIMM (Dual-Channel)
  * Inference Impact: Dictates the absolute performance ceiling for token-generation, delivering a shared theoretical bandwidth of 89.6 GB/s (translating to a real-world speed of ~12–15 tokens per second on 7B/8B models).
  * VRAM Topology: Carved out manually via the motherboard BIOS's UMA Frame Buffer Size (set to 4GB or 8GB) to allocate hardware-addressable memory space directly to the iGPU.
* Storage: 512GB Kioxia BG6 NVMe M.2 SSD (KBG60ZNV512G).
  * Interface and Performance: PCIe Gen 4.0 x4 link, delivering sequential read speeds up to ~4,600 MB/s. It leverages Host Memory Buffer (HMB) caching architecture.
  * AI Relevance: Handles initial Model Deployment Latency. The ~4.6 GB/s Gen 4 throughput guarantees that 4-bit quantized 7B/8B model weights (~4.5GB–5.5GB files) are read from non-volatile flash memory and fully populated into the system RAM buffer in under 2 seconds, neutralizing cold-start application lag. 


## Operating System

I am running Ubuntu Server 24.04 LTS on my AI rig currently. Any Linux distribution will work for a parallel setup. Note, if you use a different distribution, many commands below and file paths will be different, so if you want to implement the optimizations described, you will have to adapt them.

It is perfectly valid and has some advantages to use Windows 11, but the OS has more overhead both for memory and CPU/GPU, so you would likely need more resources. MacOS is also an execellant option, but requires Apple hardware, so won't run on the equivalant hardware to my AI rig. The rest of this doc applies to Linux, not Windows or MacOS, so you'll want to research optimizations under those operationg systems and/or figure out what equivalent to the things listed here are.

### OS Installation

1. Download the ISO image (choose the server version, not the desktop version):

[Ubuntu Server Download](https://ubuntu.com/download/server)

2. Burn the image to a USB stick at least 8GB size. I use dd to burn it on Linux on my laptop. There are plenty of burn software for Linux, Windows, and MacOS if you'd prefer something more user-friendly.


    sudo su -                                           # Change to root, if you haven't already.
    lsblk                                               # Find the USB drive, it will usually be something like sdb or sdc.
    umount /dev/sdb                                     # Unmount the drive based on the name you found above.
    dd if=ubuntu-24.04-server-amd64.iso of=/dev/sdb     # Again, use the device name you found. If the iso isn't in the current directory, use the full path.

3. Remove the USB stick, and on the AI rig, insert it. Make sure the BIOS is set to boot from it, then boot.

4. If prompted, choose install.

5. Choose your language and keyboard layout.

6. When prompted for the type, choose Ubuntu Server (minimized).

7. Configure your network connection. I have a complex home network, and run the box on an isolated network where it can connect to the internet and other things can connect to it locally, but it can't connect to them.

8. You can play with the disk configuration for what suits you, for security or ease of use. For this, I went simple, LVM with one volume, I used half but can expand later if needed, giving me flexibility. I also went with swap file instead of swap volume, for dynamic resizing.

9. Create your user profile. This is the primary unprivileged user with sudo privileges.

10. When prompted, choose install OpenSSH Server, this is how you'll manage the box remotely.

11. Skip the Featured Server Snaps, we want lean, no need to add any snaps.

After boot, you should have a running system you can log into locally or over ssh. There is no GUI, it's installed command line online. X Windows adds overhead.


## Software

I have both Ollama and Lemonade installed, and Gromrik is compatible with both (if you hit the Ollama compatible endpoint of Lemonade). However, on my AI rig, the queries the harness submits time out with Lemonade, so I'm using Ollama exclusively currently.

### Utils and Prerequisite Installation:

First, we'll install packages needed for optimization and management, and prereqs for the main packages:

    sudo su -                                                                                                      # Change to root if you aren't already.
    apt update                                                                                                     # Update the package list.
    apt upgrade                                                                                                    # Upgrade all packages to the newest version.
    apt install -y \
      net-tools \                                                                                                  # Because netstat is useful.
      nginx \                                                                                                      # This will be our proxy server in front of the various services.
      cpuinfo \                                                                                                    # Useful for seeing what's going on.
      nvme-cli \                                                                                                   # Disk information.
      vulkan-tools \                                                                                               # We'll be running ollama in Vulkan mode due to the GPU present.
      cpufrequtils                                                                                                 # For setting high responsive scaling for the CPU.
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg   # GPG for Docker install.
    chmod a+r /etc/apt/keyrings/docker.gpg                                                                         # Fix permissions.
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \                         # Add the Apt repo.
      https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") \
      stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    apt update                                                                                                     # Update the package list.
    apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y                # Install Docker.

### Ollama Installation

This is the core of the system, unless Lemonade is used instead.

    curl -fsSL https://ollama.com/install.sh | sh    # Install from the official build.
    ollama serve                                     # Start it, we'll tune it later.

### Lemonade Installation

Same drill.

    add-apt-repository ppa:lemonade-team/stable      # Add the official repo.
    apt install lemonade-server                      # Install Lemonade package.

### Open WebUI Installation

While Open WebUI isn't needed for Gromrik, it is needed if you want a direct web-based chat client or if you want to manage models and such from a GUI instead of command line.

    docker pull ghcr.io/open-webui/open-webui:main                                                                  # Pull the Open WebAI image.
    docker run -d --network=host -v open-webui:/app/backend/data -e OLLAMA_BASE_URL=http://127.0.0.1:11434 \        # Start the container.
      --name open-webui --restart always ghcr.io/open-webui/open-webui:main

You should be able to connect on Open WebUI on http://<HOST_NAME>:8080 or http://<IP_ADDRESS>:8080. I'm not including configuration for it in this doc. Note that if you want to use HTTPS (TLS encryption) instead of HTTP (unencrypted), best approach is to set that up in Nginx so you can use one cert and setup for Open WebUI, Ollama, and Lemonade. For my local setup, I'm not currently worrying about that.


### Configure Nginx Reverse Proxy

We will use Nginx as a reverse proxy in front of Ollama and Lemonade. I also have it configured for a front end to OpenClaw, but there aren't enough resources to actually use OpenClaw, so skipping that here.

Save these files, replacing "<IP_ADDRESS>" and "<HOST_NAME>" with the ones in your setup:

/etc/nginx/conf.d/ollama.conf
```nginx
server {
  listen <IP_ADDRESS>:11434;
  server_name <HOST_NAME>;
  client_max_body_size 100M;

  location / {
    proxy_pass http://localhost:11434;
    proxy_set_header Host localhost;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
          
    # Required for WebSocket support
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_set_header Origin "";
    proxy_set_header Referer "";
          
    # Recommended timeouts for Ollama
    proxy_connect_timeout 60s;
    proxy_send_timeout 60s;
    proxy_read_timeout 36000s; # Increased timeout for long generations

    # Optimize for streaming tokens
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    proxy_buffering off;
  }
}
```

/etc/nginx/conf.d/lemonade.conf
```nginx
server {
  listen <IP_ADDRESS>:13305;
  server_name <HOST_NAME>
  client_max_body_size 100M;

  proxy_buffering off;
  proxy_cache off;
  chunked_transfer_encoding on;
  proxy_connect_timeout 600s;
  proxy_send_timeout 600s;
  proxy_read_timeout 36000s;
  keepalive_timeout 600s;

  proxy_set_header Host $host;
  proxy_set_header X-Real-IP $remote_addr;
  proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  proxy_set_header X-Forwarded-Proto $scheme;

  # Front End Web Interface
  location / {
    proxy_pass http://127.0.0.1:13305;
    proxy_http_version 1.1;
    proxy_set_header Connection "";
  }

  # WebSocket Legacy
  location /ws {
    proxy_pass http://127.0.0.1:9000;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }

  # Ollama Compatible
  location /api/ {
    proxy_pass http://127.0.0.1:13305;
    proxy_http_version 1.1;
    proxy_set_header Connection "";
  }

  # OpenAI Rest
  location /v1/ {
    proxy_pass http://127.0.0.1:13305;
    proxy_http_version 1.1;
    proxy_set_header Connection "";
  }
}
```

   nginx -t                   # Test the config.
   systemctl restart nginx    # Restart NGINX to load the configs.

The primary port for Ollama is on 11434, and primary for Lemonade is 13305. These are the ports Gromrik will connect to, depending on which you use. Note we use the /api endpoint with Lemonade for Ollama compatibility.


## Optimization

### Overrides for Ollama

The following overrides are for optimizing Ollama:

/etc/systemd/system/ollama.service.d/override.conf
```ini
[Service]
# Keeps the loaded model in memory for 24 hours to prevent reload delays.
Environment="OLLAMA_KEEP_ALIVE=24h"
# Restricts concurrent requests to one to prevent splitting shared memory context.
Environment="OLLAMA_NUM_PARALLEL=1"
# Limits active memory residency to one model to avoid crashing 16GB systems.
Environment="OLLAMA_MAX_LOADED_MODELS=2"
# Activates efficient attention algorithms to drastically lower memory usage during context execution.
Environment="OLLAMA_FLASH_ATTENTION=1"
# Enables disk caching of compiled Vulkan shaders to drastically accelerate subsequent startups.
Environment="MESA_SHADER_CACHE_DISABLE=false"
# Allocates a maximum of 4 gigabytes on disk for the compiled shader storage.
Environment="MESA_SHADER_CACHE_MAX_SIZE=4G"
# Explicitly forces Ollama to use the Vulkan compute backend for the iGPU.
Environment="OLLAMA_VULKAN=1"
# Allows API access from any origin or domain web interface for flexibility.
Environment="OLLAMA_ORIGINS=*"
# Raises the maximum allowed open files to prevent crashes during large model loading.
LimitNOFILE=65535
# Allows locking unlimited RAM to prevent the system from swapping model weights.
LimitMEMLOCK=infinity
```
    systemctl daemon-reload
    systemctl start ollama.service

### Overrides for Lemonade

The following overrides are for optimizing Lemonade:

/etc/systemd/system/lemond.service.d/override.conf
```ini
[Service]
# Raises the maximum allowed open files to prevent crashes during large model loading.
LimitNOFILE=65535
# Allows locking unlimited RAM to prevent the system from swapping model weights.
LimitMEMLOCK=infinity
# Explicitly forces Lemonade to use the Vulkan backend via llama.cpp for hardware compute.
Environment="LEMONADE_LLAMACPP=vulkan"
# Tells the graphics driver layer to strictly target the open-source AMD RADV driver pipeline.
Environment="AMD_VULKAN_ICD=RADV"
```
    systemctl daemon-reload
    systemctl start ollama.service

### System Optimizations

A few changes to how memory is managed:

/etc/sysctl.conf
```ini
vm.swappiness=10
vm.overcommit_memory=1
```
    sysctl -p
### Disable Energy Saving

Disable power saving on the NIC, as this can break operations in progress, causing long hangs.

/etc/systemd/system/disable-eee.service
```ini
[Unit]
Description=Disable Energy Efficient Ethernet Power Saving
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/sbin/ethtool --set-eee enp1s0 eee off
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
```
    systemctl daemon-reload
    systemctl start disable-eee.service

Also disable udev network power saving:

/etc/udev/rules.d/99-disable-network-powersave.rules
```bash
ACTION=="add", SUBSYSTEM=="net", KERNEL=="*", RUN+="/bin/sh -c 'echo on > /sys/class/net/%k/device/power/control'"
```
    udevadm control --reload-rules
    udevadm trigger

### CPU Max Performance Mode

Force all CPU cores into maximum performance mode across system restarts.

```bash
echo 'GOVERNOR="performance"' | sudo tee /etc/default/cpufrequtils
systemctl restart cpufrequtils
```

### Disable Unneeded Services

We'll disable some things this setup doesn't use. Obviously if you do need some of these, don't disable them.

```bash
systemctl disable --now avahi-daemon
systemctl disable --now bluetooth.target
systemctl disable --now iscsid.socket
```

### Disable Wifi

Wifi, even inactive, will increase interrupts and heat, doing scans. It is highly recommended to only use hard wired network on the AI rig. If you have to use wifi, skip this section.

1. Determine what wifi interfaces you have, if any:
```bash
ip link | grep -o 'wl[^:]*'
```
2. Edit the files in /etc/netplan
3. Delete any section called `wifis:` and any lines containing the interfaces from the previous command.
4. Apply the modified plan:
```bash
netplan apply
```
5. Identify if you have any active wireless drivers:
```bash
sudo lshw -C network | grep -A 3 "Wireless" | grep "configuration:"
```
6. Create a file called /etc/modprobe.d/disable-wifi.conf with one line for each driver replacing "<DRIVER_NAME>" found:
```ini
blacklist <DRIVER_NAME>
```
7. Stop the WPA service and stop the kernel modules:
```bash
systemctl disable --now wpa_supplicant
WIFI_DRV=$(sudo lshw -C network | grep -A 3 "Wireless" | grep -o 'driver=[^ ]*' | cut -d= -f2)
modprobe -r $WIFI_DRV
```

