#!/bin/sh

# Copyright © Schubert Anselme <schubert@anselm.es>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

set -eux

: "${CROSS_COMPILE:=riscv64-linux-gnu-}"
: "${OPENSBI:=firmware/opensbi-riscv64.bin}"

# build uboot
cd build/uboot

CROSS_COMPILE="${CROSS_COMPILE}" OPENSBI="${OPENSBI}" make -j qemu-riscv64_spl_defconfig
CROSS_COMPILE="${CROSS_COMPILE}" OPENSBI="${OPENSBI}" make -j "$(nproc)"

# copy uboot to firmware/
cp -f spl/u-boot-spl.bin "${HOME}/"

# copy itb to data/
cp -f u-boot.itb "${HOME}/"
