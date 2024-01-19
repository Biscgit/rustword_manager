# This is the Rustword-Manager,

a password-safe coded entirely in Rust! It is work in progress but contains the most important features as of now:
- Setting up a master-password for the whole database
- Creation and deletion of Username-Password Entries and SSH-keypairs
- Selecting inputs and copying inputs to clipboard without showing them as plaintext

# HOW TO INSTALL

To use the RustwortManager, you will need to have SQLCipher installed (https://www.zetetic.net/sqlcipher/). This program uses the Community-version. 
If this is not deployed as an executable, you will also need to install the Rust compiler (https://www.rust-lang.org/tools/install) and git (https://git-scm.com/downloads).

Now, with all requirements installed:

1. Open the terminal (cmd on Windows) and move to the directoy/folder you want to install the program in.
2. Install our software using by typing `git clone https://github.com/Biscgit/rustword_manager.git`.
3. Run with `cargo run --release`

# HOW TO USE

You can run the source code via
    `cargo run --bin rustword_manager --release`
through the terminal. The `--release` part is optional but without that, the login into the database will take significantly longer.
Upon running the program for the first time, you will need to enter a master-password fulfilling the following minimum requirements:
- One uppercase letter
- One lowercase letter
- One digit
- One special character
- Ten characters long
A value that shows the an approximation for the strength of your password is also presented.
! IMPORTANT ! Pressing TAB in this screen will show your currently entered password! Use with caution!

Using TAB you can switch between the list of current entries and a creation prompt. As of right now, you can create Username-Password entries, SSH-keypair entries with an associated website and notes.

On the New Entry screen, you can navigate through the template options. Using right-arrow, you can select a template.
Press TAB to hide/unhide an input.
Press down-arrow or up-arrow to move across input fields. Pressing ENTER also moves you down by one field.
Press ENTER when hovering over "Insert" to create the entry.
Press ESC to leave the creation mask.

On the Credentials screen, press up-arrow/down-arrow to move across created entries. You can use the filter-textbar at the bottom as a filter.
Press right-arrow to select an entry and left-arrow to move back to the entry-selection.
Press up-arrow or down-arrow to move through a currently selected entry.
Press C to copy an entry to your clipboard.
Press ENTER twice while hovering over the "Delete Entry" button to delete the currently selected entry.

# SECURITY IMPLEMENTATIONS

The database is encrypted while on the hard-drive. The decryption key is passed to SQLCipher via key-derivation using Argon2 and a salt-value generated from SQLCipher. SQLCipher decryptes the database with its own derived key from the input using PBKDF2.
During runtime, all entries are encrypted using AES-256-GCM until they are used. We use the key derived from Argon2.
The key itself is encrypted in the RAM using Rust's shielded package. It is inaccessible for an outside attacker.
