# CIC-to-NetFlow Project

In this project, we convert CIC datasets (in PCAP and CSV) to categorized NetFlow v5 files.

- [CIC-to-NetFlow Project](#cic-to-netflow-project)
  - [Getting Started for Users](#getting-started-for-users)
  - [Steps to Process CIC-IDS-2017](#steps-to-process-cic-ids-2017)
  - [Steps to Process CIC-DDoS-2019](#steps-to-process-cic-ddos-2019)
  - [Getting Started for Developers](#getting-started-for-developers)
  - [Appendix](#appendix)
    - [NetFlow to Sessions](#netflow-to-sessions)
    - [Mapping of CIC-IDS-2017 to NetFlow v5](#mapping-of-cic-ids-2017-to-netflow-v5)
    - [Mapping of CIC-DDoS-2019 to NetFlow v5](#mapping-of-cic-ddos-2019-to-netflow-v5)
  - [Contribution Notes](#contribution-notes)

## Getting Started for Users

1. Download this repository with [git](https://git-scm.com/install).
   ```shell
   git clone https://github.com/boray-tw/cic2nf/
   cd cic2nf
   ```
2. Install [Docker](https://docs.docker.com/engine/install) and Bash ([Cygwin terminal](https://www.cygwin.com/install.html) in Windows).
3. Download `csv_to_nf` and `label_nf` binaries of your platform from the latest [release](https://github.com/boray-tw/cic2nf/releases). (Assumes running in Linux with AMD64/x86-64 architecture in the following paragraphs.)

## Steps to Process CIC-IDS-2017

1. Download and extract [CIC-IDS-2017](http://cicresearch.ca/CICDataset/CIC-IDS-2017/Dataset/CIC-IDS-2017/CSVs/GeneratedLabelledFlows.zip) CSV files as `datasets/2017/01-csv/*.csv`. ([info](https://www.unb.ca/cic/datasets/ids-2017.html))
2. Download [CIC-IDS-2017](http://cicresearch.ca/CICDataset/CIC-IDS-2017/Dataset/CIC-IDS-2017/PCAPs/) PCAP files as `datasets/2017/02-pcap/*.pcap`.
3. Convert CSV to NetFlow files. (Replace `1-monday` and `Monday` with other days for the next few runs.)
   ```shell
   csv_to_nf \
		--name CIC-IDS-2017 \
		--output_dir datasets/2017/11-nfs-from-csv/1-monday \
		$(ls datasets/2017/01-csv/Monday*.csv)
   ```
4. Convert PCAP to NetFlow files.
   ```shell
   cd pcap-to-netflow

   # update the following configs in `run-me.sh`
   # INPUT_DIR="$PWD/../datasets/2017/02-pcap/01-12"
   # OUTPUT_DIR="$PWD/../datasets/2017/12-nf-from-pcap"

   # update the following configs in `entry-inside-container.sh`
   # TO_MERGE_ALL_PCAP="n"
   # PCAP_FILES=$(find $PCAP_IN_DIR -type f)

   bash ./run-me.sh
   ```
5. Label NetFlow files.
   <!-- TODO -->

## Steps to Process CIC-DDoS-2019

1. Download and extract [CIC-DDoS-2019](http://cicresearch.ca/CICDataset/CICDDoS2019/Dataset/CSVs/) CSV files as `datasets/2019/01-csv/{01-12,03-11}/*.csv`. ([info](https://www.unb.ca/cic/datasets/ddos-2019.html))
2. Download and extract [CIC-DDoS-2019](http://cicresearch.ca/CICDataset/CICDDoS2019/Dataset/PCAPs/) PCAP files to `datasets/2019/02-pcap/{01-12,03-11}/`.
3. Convert CSV to NetFlow files. (Replace two `03-11` with `01-12` for the next run.)
   ```shell
   csv_to_nf \
		--name CIC-DDoS-2019 \
		--output_dir datasets/2019/11-nfs-from-csv/03-11 \
		$(ls datasets/2019/01-csv/03-11/*.csv)
   ```
4. Convert PCAP to NetFlow files. (Replace two `03-11` with `01-12` for the next run.)
   ```shell
   cd pcap-to-netflow

   # update the following configs in `run-me.sh`
   # INPUT_DIR="$PWD/../datasets/2019/02-pcap/01-12"
   # OUTPUT_DIR="$PWD/../datasets/2019/12-nf-from-pcap"

   # update the following configs in `entry-inside-container.sh`
   # TO_MERGE_ALL_PCAP="y"
   # MERGED_NF_FILENAME=01-12.nf
   # PCAP_FILES=$(find $PCAP_IN_DIR -type f | sort -t _ -k 3n)

   bash ./run-me.sh
   ```
5. Label NetFlow files.
   <!-- TODO -->

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

3. (Optional) Download [CIC-IDS-2017](http://cicresearch.ca/CICDataset/CIC-IDS-2017/Dataset/CIC-IDS-2017/) and/or [CIC-DDoS-2019](http://cicresearch.ca/CICDataset/CICDDoS2019/Dataset/) datasets.

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
  | ---------: | --------------------------- | --------: | --------------------- |
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

  CIC-IDS-2017 timestamp format: either `yyyy-MM-dd II:mm:ss` or `yyyy-MM-dd k:mm`, where `k` means an hour ranged in [1, 12], and `II` is zero-padded `k`.

  References:
  1. [Definition of CIC CSV datasets](https://github.com/CanadianInstituteForCybersecurity/CICFlowMeter/blob/master/ReadMe.txt)
  2. [Definition of NetFlow v5 format](https://raysquare.notion.site/231018-NetFlow-v5-format-in-HPDS-bc971d75f74f4bb6bb68d2dca55a5f15?pvs=4)
</details>



### Mapping of CIC-DDoS-2019 to NetFlow v5

<details>
  <summary>Click to expand/collapse:</summary>

  | CIC Column | CIC Header Name             | NF Column | NF Name               |
  | ---------: | --------------------------- | --------: | --------------------- |
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
  |         88 | Label                       |         - | -                     |
  |          - | -                           |         9 | QoS                   |
  |          - | -                           |        12 | # of flows            |

  CIC-DDoS-2019 timestamp format: `yyyy-MM-dd HH:mm:ss.SSSSSS`, where `ss.SSSSSS` means `ss` seconds plus zero-left-padding `SSSSSS` microseconds.
</details>

## Contribution Notes

Please always format your Rust code with VS Code extension
[rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) before committing Rust code.
