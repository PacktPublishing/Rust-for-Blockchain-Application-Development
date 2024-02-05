import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaCustom } from '../target/types/solana_custom';
import * as assert from "assert";
import * as bs58 from "bs58";

describe('solana-custom', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.Provider.env());
    const program = anchor.workspace.SolanaCustom as Program<SolanaCustom>;

    it('can send a new message', async () => {
        // Execute the "SendMessage" instruction.
        const message = anchor.web3.Keypair.generate();
        await program.rpc.sendMessage('space exploration', 'Discovering new worlds!', {
            accounts: {
                message: message.publicKey,
                author: program.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [message],
        });

        // Fetch the account details of the created message.
        const messageAccount = await program.account.message.fetch(message.publicKey);

        // Ensure it has the right data.
        assert.equal(messageAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
        assert.equal(messageAccount.topic, 'space exploration');
        assert.equal(messageAccount.content, 'Discovering new worlds!');
        assert.ok(messageAccount.timestamp);
    });


    it('can fetch all messages', async () => {
        const messageAccounts = await program.account.message.all();
        assert.equal(messageAccounts.length, 3);
    });
});
