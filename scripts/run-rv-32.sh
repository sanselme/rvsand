#!/bin/bash

# Copyright (c) 2022 Schubert Anselme <schubert@anselm.es>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

set -e

CORES=1
MEMORY="256M"
UUID="$(uuidgen)"

# FIXME: randomize port numbers
qemu-system-riscv32 \
  --uuid "${UUID}" \
  --machine virt \
  --smp "${CORES}" \
  -m "${MEMORY}" \
  -bios "${1}" \
  -kernel "${2}" \
  -device virtio-rng \
  -device virtio-blk-device,drive=hd0 \
  -drive id=hd0,if=none,format="${3}",file="${4}" \
  -device virtio-net-device,netdev=net0 \
  -netdev user,id=net1,hostfwd=tcp::10080-:80,hostfwd=tcp::10443-:443 \
  "${5}"
