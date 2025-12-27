# Converting PCAP Files to NetFlow V5 Files

## Simplified workflow

1. For Linux, install [Docker Engine](https://docs.docker.com/engine/install/) or [Docker Desktop](https://docs.docker.com/desktop/install/windows-install/). For example, in Ubuntu, follow the instructions in [this section](https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository), and follow the [post-installation steps](https://docs.docker.com/engine/install/linux-postinstall/).

2. For Windows, install [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) or supervisors such as [VirtualBox](https://www.virtualbox.org/) or [VMWare](https://www.vmware.com/), and then follow the instructions above. Otherwise, install [Docker Desktop](https://docs.docker.com/desktop/install/windows-install/).

3. Put all [PCAP](https://en.wikipedia.org/wiki/Pcap) files under `input` of the current directory.

4. Check the configurations in `entry-inside-container.sh`. In particular, you usually have to modify `TO_MERGE_ALL_PCAP`, and make sure the content of `PCAP_FILES` is sorted if you set `TO_MERGE_ALL_PCAP` to `y`. (For Windows, you will see the content of `PCAP_FILES` when running `docker run ...` later.)

5. For Linux, run `run-me.sh`. It will build the Docker image according to `Dockerfile`, which puts `entry-inside-container.sh` inside the image, and then convert PCAP files to NetFlow v5 files inside the container.

        chmod u+x run-me.sh
        ./run-me.sh

6. For Windows, modify the following commands at your preference, or just run them without modification.

        # in CMD
        # FIXME: these commands are not tested

        # after stopping the container `pcap-to-nf`
        # and removing the image `pcap-to-nf`,
        # build the image
        docker build -t "pcap-to-nf:latest" .

        # make a directory for the output
        mkdir output

        # run the following command in one line
        docker run --rm -it --name pcap-to-nf --mount type=bind,src=%cd%\input,target=/pcap-to-nf/input --mount type=bind,src=%cd%\output,target=/pcap-to-nf/output "pcap-to-nf:latest"

7. Check the output under `output` directory.
