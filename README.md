# Solana CLI Airdrop and Balance Checker

## Overview

This Rust program interacts with the Solana blockchain, allowing users to:

- Generate a keypair
- Request airdrops of SOL tokens
- Check their account balance

It connects to the Solana Devnet and provides a simple CLI interface for these functions.

## Features

- Generate a new keypair or use an existing public key.
- Request an airdrop of SOL tokens.
- Check the current balance of the specified account.

## Prerequisites

- Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- Access to the Solana blockchain (this program connects to the Devnet).

## Usage Instructions

1. **Public Key Input**
   - When prompted, specify whether you have an existing public key.
     - Type `yes` to input your public key.
     - Type `no` to generate a new keypair.

2. **Airdrop SOL**
   - Choose option `1` to request an airdrop.
   - Enter the amount of SOL you wish to request (between 0 and 1).

3. **Check Balance**
   - Choose option `2` to check your current balance.

4. **Exit the Application**
   - Choose option `3` to exit the program.

## Code Explanation

### Main Functionality
The main function handles user input and provides options for interacting with the Solana blockchain.

### Key Functions
- **create_key_pair**: Generates a new keypair for the user.
- **req_airdrop**: Requests an airdrop of SOL tokens.
- **check_account_balance**: Fetches and displays the current balance of the user's account.
- **lamports_transaction**: Facilitates transferring SOL between accounts (not used in main flow but available for future use).

## Example Output
```
======================= Welcome to Solana CLI =======================
 Rpc is connected to devnet by default
 Do you have a public key? (yes or no): yes
 Paste your Public-Key: <your_public_key>

 You can choose the options:

 Click if you want an airdrop of Solana
 Click if you want to check the current balance
 Click if you want to exit. how to writ this in code
```
