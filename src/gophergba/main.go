// Copyright (c) 2022 Schubert Anselme <schubert@anselm.es>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

package main

import (
	"machine"
	"runtime/interrupt"
)

func main() {
	// Set up the display
	display.Configure()

	// Register display status
	regDispStat.SetBits(1<<3 | 1<<4)

	// Display Gopher text message and draw our Gophers
	drawGophers()

	// Creates an interrupt that will call the "update" function
	// bellow, hardware way to display things on the screen
	interrupt.New(machine.IRQ_VBLANK, update).Enable()

	// TODO: Infinite loop to avoid exiting the application
	for {
	}
}
