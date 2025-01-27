# Solana CLI Airdrop and Balance Checker

## Overview

<p> This Rust program interacts with the Solana blockchain, allowing users to generate a keypair, request airdrops of SOL tokens, and check their account balance. It connects to the Solana Devnet and provides a simple command-line interface for user interaction.
</p>

## Features 
<li>Generate a new keypair or use an existing public key.</li>
<li>Request an airdrop of SOL tokens.</li>
<li>Check the current balance of the specified account.</li>

## Prerequisites

<li>Rust installed on your machine. You can install it from rust-lang.org.</li>
<li>Access to the Solana blockchain (this program connects to the Devnet).</li>

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

======================= Welcome to solana CLI Rpc is connected to devnet by default ======================= <br/>
<br/>
Do you have public key ? (yes or no)
<br/>
yes
<br/>
Paste your Public-Key
<br/>
<your_public_key>
<br/>
You can choose the options:
<br/>
1.Click if you want airdrop solana
<br/>
2.Click if you check current balance
<br/>
3.Click if you want to exit
<br/>


