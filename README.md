# CIC-to-NetFlow Project

In this project, we convert CIC datasets (in PCAP and CSV) to categorized NetFlow v5 files.

- [CIC-to-NetFlow Project](#cic-to-netflow-project)
  - [Getting Started for Developers](#getting-started-for-developers)
  - [Appendix](#appendix)
    - [NetFlow to Sessions](#netflow-to-sessions)
    - [Mapping of CIC-IDS-2017 to NetFlow v5](#mapping-of-cic-ids-2017-to-netflow-v5)
    - [Mapping of CIC-DDoS-2019 to NetFlow v5](#mapping-of-cic-ddos-2019-to-netflow-v5)
  - [Contribution Notes](#contribution-notes)

## Getting Started for Developers

1. Install dependencies:
   * Debian/Ubuntu-based distributions or Windows Subsystem Linux 2 (WSL2):
     ```shell
     sudo apt-get install -y make

     # option 1: native compilation
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

     # option 2: compilation in a container
     # for more info: https://docs.docker.com/engine/install
     ```
   * Windows 11 (native):
     1. Install [Cygwin](https://www.cygwin.com/install.html) and its `make` package.
     2. Install [Rust](https://rustup.rs/).
        <details>
          <summary>Ray's preferred options:</summary>
          1. Toolchain: GNU rather than Visual Studio (MSVC).
          2. Custom installation.
          3. Platform: `x86_64-pc-windows-gnu`
          4. Toolchain: `stable`
          5. Tools: `default`
          6. Modify `PATH`
        </details>

2. Download this repository with [git](https://git-scm.com/install).
   ```shell
   git clone https://github.com/boray-tw/cic2nf/
   cd cic2nf
   ```

3. (Optional) Download [CIC-IDS-2017](http://cicresearch.ca/CICDataset/CIC-IDS-2017/Dataset/CIC-IDS-2017/) ([info](https://www.unb.ca/cic/datasets/ids-2017.html)) and/or [CIC-DDoS-2019](http://cicresearch.ca/CICDataset/CICDDoS2019/Dataset/) ([info](https://www.unb.ca/cic/datasets/ddos-2019.html)) datasets.

4. (Optional) In `makefile`, update the paths like `INPUT_DIR_HOST`, `OUTPUT_DIR_HOST`, and update the commands in the `debug` target.

5. (Optional) Launch a Docker container.
   ```shell
   make dev
   ```

6. Build the binaries.
   ```shell
   make build
   ```

7. (Optional) Run the binaries with debugging information.
   ```shell
   make debug
   ```

8. (Optional) Clean up.
   ```shell
   make clean
   ```

9.  (Optional) Check the help messages of help targets.
   ```shell
   make info
   ```

## Appendix

### NetFlow to Sessions

Please refer to [this](https://raysquare.notion.site/230801-Run-BotCluster-2-v1-1-532d58675a414184b6284b966c738883?pvs=4) Notion note to convert NetFlow to session using [BotCluster 2](https://github.com/HPDS/botnet-detection-algorithm/tree/master/BotCluster2).

### Mapping of CIC-IDS-2017 to NetFlow v5

<details>
  <summary>Click to expand/collapse:</summary>

  | CIC Column | CIC Header Name             | NF Column | NF Name               |
  | ----------:| --------------------------- | ---------:| --------------------- |
  |          2 | Source IP                   |         4 | Source IP             |
  |          3 | Source Port                 |         5 | Source port           |
  |          4 | Destination IP              |         6 | Destination IP        |
  |          5 | Destination Port            |         7 | Destination Port      |
  |          6 | Protocol                    |         3 | Protocol              |
  |          7 | Timestamp                   |         1 | Timestamp             |
  |          8 | Flow Duration (us)          |         2 | Duration (s)          |
  |          9 | Total Fwd Packets           |        10 | # of packets          |
  |         10 | Total Backward Packets      |        10 | # of packets          |
  |         11 | Total Length of Fwd Packets |        11 | # of bytes in layer 3 |
  |         12 | Total Length of Bwd Packets |        11 | # of bytes in layer 3 |
  |         37 | Fwd PSH Flags               |         8 | Flags                 |
  |         38 | Bwd PSH Flags               |         8 | Flags                 |
  |         39 | Fwd URG Flags               |         8 | Flags                 |
  |         40 | Bwd URG Flags               |         8 | Flags                 |
  |         41 | Fwd Header Length           |        11 | # of bytes in layer 3 |
  |         42 | Bwd Header Length           |        11 | # of bytes in layer 3 |
  |         50 | FIN Flag Count              |         8 | Flags                 |
  |         51 | SYN Flag Count              |         8 | Flags                 |
  |         52 | RST Flag Count              |         8 | Flags                 |
  |         53 | PSH Flag Count              |         8 | Flags                 |
  |         54 | ACK Flag Count              |         8 | Flags                 |
  |         55 | URG Flag Count              |         8 | Flags                 |
  |         56 | CWR Flag Count              |         8 | Flags                 |
  |         57 | ECE Flag Count              |         8 | Flags                 |
  |         85 | Label                       |         - | -                     |
  |          - | -                           |         9 | QoS                   |
  |          - | -                           |        12 | # of flows            |

  CIC-IDS-2017 timestamp format: either `yyyy-MM-dd II:mm:ss` or `yyyy-MM-dd II:mm`, where `II` means an hour ranged in [00, 11].

  References:
  1. [Definition of CIC CSV datasets](https://github.com/CanadianInstituteForCybersecurity/CICFlowMeter/blob/master/ReadMe.txt)
  2. [Definition of NetFlow v5 format](https://raysquare.notion.site/231018-NetFlow-v5-format-in-HPDS-bc971d75f74f4bb6bb68d2dca55a5f15?pvs=4)
</details>



### Mapping of CIC-DDoS-2019 to NetFlow v5

<details>
  <summary>Click to expand/collapse:</summary>

  | CIC Column | CIC Header Name             | NF Column | NF Name               |
  | ----------:| --------------------------- | ---------:| --------------------- |
  |          3 | Source IP                   |         4 | Source IP             |
  |          4 | Source Port                 |         5 | Source port           |
  |          5 | Destination IP              |         6 | Destination IP        |
  |          6 | Destination Port            |         7 | Destination Port      |
  |          7 | Protocol                    |         3 | Protocol              |
  |          8 | Timestamp                   |         1 | Timestamp             |
  |          9 | Flow Duration (us)          |         2 | Duration (s)          |
  |         10 | Total Fwd Packets           |        10 | # of packets          |
  |         11 | Total Backward Packets      |        10 | # of packets          |
  |         12 | Total Length of Fwd Packets |        11 | # of bytes in layer 3 |
  |         13 | Total Length of Bwd Packets |        11 | # of bytes in layer 3 |
  |         38 | Fwd PSH Flags               |         8 | Flags                 |
  |         39 | Bwd PSH Flags               |         8 | Flags                 |
  |         40 | Fwd URG Flags               |         8 | Flags                 |
  |         41 | Bwd URG Flags               |         8 | Flags                 |
  |         42 | Fwd Header Length           |        11 | # of bytes in layer 3 |
  |         43 | Bwd Header Length           |        11 | # of bytes in layer 3 |
  |         51 | FIN Flag Count              |         8 | Flags                 |
  |         52 | SYN Flag Count              |         8 | Flags                 |
  |         53 | RST Flag Count              |         8 | Flags                 |
  |         54 | PSH Flag Count              |         8 | Flags                 |
  |         55 | ACK Flag Count              |         8 | Flags                 |
  |         56 | URG Flag Count              |         8 | Flags                 |
  |         57 | CWR Flag Count              |         8 | Flags                 |
  |         58 | ECE Flag Count              |         8 | Flags                 |
  |         87 | Label                       |         - | -                     |
  |          - | -                           |         9 | QoS                   |
  |          - | -                           |        12 | # of flows            |

  CIC-DDoS-2019 timestamp format: `yyyy-MM-dd HH:mm:ss.SSSSSS`, where `ss.SSSSSS` means `ss` seconds plus zero-left-padding `SSSSSS` microseconds.
</details>

## Contribution Notes

Please always format your Rust code with VS Code extension
[rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) before committing Rust code.
