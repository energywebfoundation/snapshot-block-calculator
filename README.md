<!--
 Copyright 2021, 2022 Energy Web Foundation
 
 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <http://www.gnu.org/licenses/>.
-->

# Snapshot Block Calculator

This program is to allow EnergyWeb to fairly select blocks at which staking snapshots will be taken.

EnergyWeb's approach for selection of the snapshot block is as follows:
1. Select a blocknumber range (start and end blocknumbers) from which a snapshot block is to be calculated
1. Obtain the blockhash of a block to use as a seed value.
   If EnergyWeb commits to use the blockhash of a future block, this is a source of randomness which is difficult enough to influence or predict.
1. Calculate a blocknumber within the random using this [program](./src/lib.rs)

## Build

`cargo build`

## Run

`cargo run`