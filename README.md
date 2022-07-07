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
   If EnergyWeb commits to use the blockhash of a future block using [SnapshotSeedRegistration contract](https://explorer.energyweb.org/address/0x4A0F475c59c9453B29c66548DB86f6557a75F448/transactions)
1. Calculate a blocknumber within the random using this [program](./src/lib.rs)

## Calculated Snapshot Blocks

- Snapshot 1: [18059849](https://explorer.energyweb.org/block/18059849/transactions)

## Build

`cargo build`

## Run

`cargo run`