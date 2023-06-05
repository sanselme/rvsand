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
	"image/color"
	"machine"
	"runtime/volatile"
	"unsafe"
)

var (
	// KeyCodes / Buttons
	keyA         = uint16(1022)
	keyB         = uint16(1021)
	keyDown      = uint16(895)
	keyLeft      = uint16(991)
	keyLShoulder = uint16(511)
	keyRight     = uint16(1007)
	keyRShoulder = uint16(767)
	keySelect    = uint16(1019)
	keyStart     = uint16(1015)
	keyUp        = uint16(959)

	// Register display
	regDispStat = (*volatile.Register16)(unsafe.Pointer(uintptr(0x4000004)))

	// Register keypad
	regKeypad = (*volatile.Register16)(unsafe.Pointer(uintptr(0x04000130)))

	// Display from machine
	display = machine.Display

	// Screen resolution
	screenWidth, screenHeight = display.Size()

	// Colors
	black = color.RGBA{}
	white = color.RGBA{255, 255, 255, 255}
	green = color.RGBA{0, 255, 0, 255}
	red   = color.RGBA{255, 0, 0, 255}

	// Google colors
	gBlue   = color.RGBA{66, 163, 244, 255}
	gRed    = color.RGBA{219, 68, 55, 255}
	gYellow = color.RGBA{244, 160, 0, 255}
	gGreen  = color.RGBA{15, 157, 88, 255}

	// Coordinates
	x int16 = 100 // TODO: horizontally center
	y int16 = 100 // TODO: vertically center
)
