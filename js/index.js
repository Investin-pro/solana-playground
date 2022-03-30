import {
    Connection,
    clusterApiUrl,
    PublicKey,
    Keypair,
    Transaction,
    TransactionInstruction,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction} from '@solana/web3.js';
import { nu64, struct, u8 } from "buffer-layout";

const programId = new PublicKey('...');

const devConnection = new Connection(clusterApiUrl('devnet'), 'processed');

const account1 = Keypair.fromSecretKey(new Uint8Array([
    // ...
]))

const main = async () => {
    try {
        const airdrop = await devConnection.requestAirdrop(account1.publicKey, LAMPORTS_PER_SOL);

        await devConnection.confirmTransaction(airdrop);

        const transaction = new Transaction();

        const dataLayout = struct([u8("instruction"), nu64('amount')])
        const data = Buffer.alloc(dataLayout.span)
        dataLayout.encode(
            { instruction: 0, amount: 10*1000000000 },
            data
        )

        const instruction = new TransactionInstruction({
            keys: [
                { pubkey: account1.publicKey, isSigner: true, isWritable: true }
            ],
            programId,
            data
        })

        transaction.add(instruction);

        transaction.feePayer = account1.publicKey

        // const x = await simulateTransaction(devConnection, transaction, 'processed')

        const sig  = await sendAndConfirmTransaction(devConnection, transaction, [account1])
        console.log('sig :>> ', sig);
    } catch (error) {
        console.log(error)
    }
}

main()

async function simulateTransaction(
    connection,
    transaction,
    commitment,
) {
    // @ts-ignore
    transaction.recentBlockhash = await connection._recentBlockhash(
        // @ts-ignore
        connection._disableBlockhashCaching,
    );

    const signData = transaction.serializeMessage();
    // @ts-ignore
    const wireTransaction = transaction._serialize(signData);
    const encodedTransaction = wireTransaction.toString('base64');
    const config = { encoding: 'base64', commitment };
    const args = [encodedTransaction, config];

    // @ts-ignore
    const res = await connection._rpcRequest('simulateTransaction', args);
    if (res.error) {
        throw new Error('failed to simulate transaction: ' + res.error.message);
    }
    return res.result;
}