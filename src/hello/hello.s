# Copyright © 2022 Schubert Anselme <schubert@anselm.es>
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

.align 2
.equ UART_REG_TXFIFO, 0x0
.equ UART_BASE, 0x10010000

.section .text
.global _start

_start:
  csrr t0, mhartid
  bnez t0, halt

  la sp, stack_top
  la a0, msg
  jal puts

puts:
  li t0, UART_BASE

.puts_loop: lbu t1, (a0)
  beqz t1, .puts_done

.puts_waits: lw t2, UART_REG_TXFIFO(t0)
  bltz t2, .puts_waits

  sw t1, UART_REG_TXFIFO(t0)

  addi a0, a0, 1
  j .puts_loop

.puts_done:
  ret

halt: j halt

.section .rodata
msg:
  .string "Hello, world!\n"
