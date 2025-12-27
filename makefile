SHELL = /bin/sh

BUILD_DIR = build

# host paths
#INPUT_DIR_HOST = $(PWD)/input
INPUT_DIR_HOST = /bc0/misc/datasets
OUTPUT_DIR_HOST = $(PWD)/output

# Docker paths
INPUT_DIR_DOCKER = /input
OUTPUT_DIR_DOCKER = /output

# paths used in "make debug"
# ref: https://stackoverflow.com/a/65942222/27092911
# ref: https://stackoverflow.com/a/20566812/27092911
ifeq ("$(IS_DOCKER_RUNNING)","true")
INPUT_DIR_DEBUG = $(INPUT_DIR_DOCKER)
OUTPUT_DIR_DEBUG = $(OUTPUT_DIR_DOCKER)
else
INPUT_DIR_DEBUG = $(INPUT_DIR_HOST)
OUTPUT_DIR_DEBUG = $(OUTPUT_DIR_HOST)
endif

# reference: https://gist.github.com/sighingnow/deee806603ec9274fd47
ifeq ($(OS), Windows_NT)
WIN_EXE_EXT = .exe
RM = del /F /Q
CAT = type
else
RM = rm -rf
WIN_EXE_EXT =
CAT = cat
endif

all: build

build: build_dir
	@cargo build --release
	@cp target/release/csv_to_nf$(WIN_EXE_EXT) target/release/label_nf$(WIN_EXE_EXT) $(BUILD_DIR)/
	@echo "\
	Built binaries in $(BUILD_DIR)/: \
	$$(ls src/bin | sed -e 'N;s/\n\|$$/$(WIN_EXE_EXT) /g')\
	"

info:
	@$(CAT) makefile-info.txt

build_dir:
	@mkdir -p "$(BUILD_DIR)"

# reference: https://stackoverflow.com/a/57776923
# reference: https://faun.pub/set-current-host-user-for-docker-container-4e521cef9ffc
dev:
	@mkdir -p "$(INPUT_DIR_HOST)" "$(OUTPUT_DIR_HOST)"

	@docker run --rm -it \
		-w /cic2nf \
		-v "$(PWD)":/cic2nf \
		-v "$(INPUT_DIR_HOST)":/$(INPUT_DIR_DOCKER):ro \
		-v "$(OUTPUT_DIR_HOST)":/$(OUTPUT_DIR_DOCKER) \
		-e HOST_UID=$$(id -u) \
		-e HOST_USERNAME=$$(whoami) \
		-e IS_DOCKER_RUNNING=true \
		-e force_color_prompt=yes \
		--hostname cic2nf-dev \
		--name cic2nf-dev \
		rust:1.92-slim-bookworm \
		bash -ic ' \
			apt-get update -qqy && \
			apt-get install -qqy gosu make >/dev/null && \
			src/docker/entrypoint.sh bash -i; \
		'

debug:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run --bin csv_to_nf -- \
		--name CIC-DDoS-2019 \
		--output_dir $(OUTPUT_DIR_DEBUG)/11-nfs-from-csv/03-11 \
		$$(find $(INPUT_DIR_DEBUG)/CIC-DDoS-2019/01-csv/03-11 -name '*.csv' | sort)

# RUST_BACKTRACE=1 $(BUILD_DIR)/label_nf$(WIN_EXE_EXT) \
# 	--labels input/ids-2017/nf-from-csv/5-friday
# 	input/ids-2017/nf-from-csv \
# 	input/ids-2017/nf-labeled.nf \
# 	input/ids-2017/nf-from-csv/Portmap.nf \
# 	input/ids-2017/nf-from-pcap

clean:
	@$(RM) "$(BUILD_DIR)"
	@cargo clean

.PHONY: build
