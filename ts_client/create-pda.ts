import * as web3 from "@solana/web3.js"
import "dotenv/config"
import { getKeypairFromEnvironment } from "@solana-developers/helpers"

//const payer = web3.Keypair.generate();
const payer = getKeypairFromEnvironment('KEVIN_KEY');

let connection = new web3.Connection("http://127.0.0.1:8899", 'confirmed');
const version = await connection.getVersion();
console.log('Connection to local cluster established:', version);

const  programId= new web3.PublicKey('AvJadcJAWgZVbobSCerqLz3JmWLget9GpfdA8cDFTfFv');
console.log('programId = ', programId.toBase58());

// airdrop
const airdropRequest = await connection.requestAirdrop(
    payer.publicKey,
    web3.LAMPORTS_PER_SOL,
);
await connection.confirmTransaction(airdropRequest);

const balanceLamports = await connection.getBalance(payer.publicKey);
const balance = balanceLamports / web3.LAMPORTS_PER_SOL;

console.log(`The balance of address ${payer.publicKey.toBase58()} is ${balance}, lamports = ${balanceLamports}`);

// setup pda
let [pda, bump] = await web3.PublicKey.findProgramAddress(
	[Buffer.from("vault"), payer.publicKey.toBuffer()],
	programId,
);
console.log(`bump: ${bump}, pubkey: ${pda.toBase58()}`);

const transaction = new web3.Transaction();

const instruction = new web3.TransactionInstruction({
    keys: [
    {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: true
    },
    {
    	pubkey: pda,
	isSigner: false,
	isWritable: true,
    },
    {
    	pubkey: web3.SystemProgram.programId,
	isSigner: false,
	isWritable: false,
    },
    ],
    programId,
});
transaction.add(instruction);

const signature = await web3.sendAndConfirmTransaction(
  connection, transaction, [payer]
);

console.log(`Transaction completed! Signature is ${signature}`);
