import {
  PublicKey,
  Connection,
  clusterApiUrl,
  NONCE_ACCOUNT_LENGTH,
  Keypair,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js"
import * as bs58 from 'bs58';
import { readFileSync } from "fs"
import * as BufferLayout from '@solana/buffer-layout'
import { Buffer } from 'buffer';
import { BN } from "bn.js"
import * as c from 'crypto';
const connection = new Connection(clusterApiUrl('devnet'), "confirmed")
// keypair bet account
let keyBet: Keypair
const BET_KEY = readKeypairFromPath(__dirname + '/../localnet/bet.json')
const USER1_KEY = readKeypairFromPath(__dirname + '/../localnet/user1.json')
const USER2_KEY = readKeypairFromPath(__dirname + '/../localnet/user2.json')
const USER3_KEY = readKeypairFromPath(__dirname + '/../localnet/user3.json')
const PROGRAM_ID = readKeypairFromPath(__dirname + '/../../dist/program/programbet-keypair.json')
// const BET_KEY = new PublicKey('3hkbbHiEg8tHUcPNiqQfdHNiBoUWtBqUxDTqTagHdkTd')
// const USER1_KEY = new PublicKey('EwM4EAtEkR5QK5wgtyZoS6jVowkKnNxrPMozt7RVEzU6')

function readKeypairFromPath(path: string): Keypair {
  const data = JSON.parse(readFileSync(path, "utf-8"))
  return Keypair.fromSecretKey(Buffer.from(data))
}

// instructions
let createIncrementInstruction = (): Buffer => {
  const layout = BufferLayout.struct([BufferLayout.u8('instruction')])
  const data = Buffer.alloc(layout.span)
  layout.encode({
    instruction: 0
  }, data)
  return data
}
let createDecrementInstruction = (): Buffer => {
  const layout = BufferLayout.struct([BufferLayout.u8('instruction')])
  const data = Buffer.alloc(layout.span)
  layout.encode({
    instruction: 1
  }, data)
  return data
}
let createSetValInstruction = (): Buffer => {
  const layout = BufferLayout.struct([
    BufferLayout.u8('instruction'),
    BufferLayout.ns64('xjust'),
    BufferLayout.u8('side')
  ])
  console.log(layout.span);
  const data = Buffer.alloc(layout.span)
  layout.encode({
    xjust: new BN("100000000"),
    side: "1",
    instruction: 2,
  }, data)
  return data
}

export async function createBet() {
  // const greetedAccount = await connection.getAccountInfo(BET_KEY.publicKey);
  let newBet: Keypair = Keypair.generate();

  let greetedPubkey = await PublicKey.createWithSeed(
    newBet.publicKey,
    'GREETING_SEED',
    PROGRAM_ID.publicKey,
  );

  const lamports = await connection.getMinimumBalanceForRentExemption(
    1000000,
  );

  const sig = await connection.requestAirdrop(
    newBet.publicKey,
    lamports + lamports,
  );
  await connection.confirmTransaction(sig);

  let getBalance = await connection.getBalance(newBet.publicKey)
  console.log("balance ", getBalance / 100000000);

  console.log('lamports', lamports / 100000000, 'for', 1000000, 'bytes')
  const transaction = new Transaction().add(
    SystemProgram.createAccountWithSeed({
      fromPubkey: newBet.publicKey,
      basePubkey: newBet.publicKey,
      seed: 'GREETING_SEED',
      newAccountPubkey: greetedPubkey,
      lamports,
      space: 1000000,
      programId: PROGRAM_ID.publicKey,
    }),
  );
  await sendAndConfirmTransaction(connection, transaction, [newBet]);
  console.log("newbet", greetedPubkey.toString());

}


async function main() {
  console.log(USER1_KEY.publicKey.toString())
  console.log(BET_KEY.publicKey.toString())
  const ti = new TransactionInstruction({
    keys: [
      { pubkey: USER1_KEY.publicKey, isSigner: true, isWritable: true },
      { pubkey: BET_KEY.publicKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID.publicKey,
    data: createSetValInstruction(),
  })
  const tx = await new Transaction().add(ti)
  console.dir(tx, { depth: 10 })
  const res = await sendAndConfirmTransaction(connection, new Transaction().add(ti), [Keypair.fromSecretKey(USER1_KEY.secretKey)])
  console.log(res)

  const accountInfo = await connection.getAccountInfo(BET_KEY.publicKey);
  console.dir(accountInfo);

  if (accountInfo === null) {
    throw 'Error: cannot find the greeted account';
  }


}


// main()
//   .then(() => process.exit(0))
//   .catch((err) => console.error(err))

let show = async () => {
  const layout = BufferLayout.struct([
    BufferLayout.u8('instruction'),
    BufferLayout.nu64('xjust'),
    BufferLayout.u8('side')
  ])
  let signatures = await connection.getSignaturesForAddress(new PublicKey('EwM4EAtEkR5QK5wgtyZoS6jVowkKnNxrPMozt7RVEzU6'))

  let trs = await connection.getTransactions(signatures.map(item=>item.signature))
  
  // console.dir(trs, {depth: 10})
  console.log('------------')
  let data = trs[0]?.transaction.message.instructions[0].data || ""
  let dataUint8 = bs58.decode(data)
  console.log(dataUint8)
  console.log(layout.decode(dataUint8))
}

show()