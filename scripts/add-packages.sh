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

apt install -y \
  autoconf \
  automake \
  autotools-dev \
  bc \
  bison \
  build-essential \
  curl \
  device-tree-compiler \
  flex \
  gawk \
  gcc-riscv64-linux-gnu \
  git \
  libexpat-dev \
  libgmp-dev \
  libmpc-dev \
  libmpfr-dev \
  libncurses-dev \
  libssl-dev \
  libtool \
  patchutils \
  python3-dev \
  swig \
  zlib1g-dev
