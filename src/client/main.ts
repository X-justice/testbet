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
import { readFileSync } from "fs"
import * as BufferLayout from '@solana/buffer-layout'
import { Buffer } from 'buffer';
import BN from "bn.js"
import * as c from 'crypto';
const connection = new Connection("https://api.devnet.solana.com", "confirmed")
// keypair bet account
let keyBet: Keypair
const BET_KEY = readKeypairFromPath(__dirname + '/../localnet/bet.json')
const USER1_KEY = readKeypairFromPath(__dirname + '/../localnet/user1.json')
const USER2_KEY = readKeypairFromPath(__dirname + '/../localnet/user2.json')
const USER3_KEY = readKeypairFromPath(__dirname + '/../localnet/user3.json')
const PROGRAM_ID = readKeypairFromPath(__dirname + '/../../dist/program/programbet-keypair.json')


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

async function main() {

  const ti = new TransactionInstruction({
    keys: [
      { pubkey: USER1_KEY.publicKey, isSigner: true, isWritable: true },
      { pubkey: BET_KEY.publicKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID.publicKey,
    data: createSetValInstruction(),
  })
  const res = await sendAndConfirmTransaction(connection, new Transaction().add(ti), [USER1_KEY])
  console.log(res)

  const accountInfo = await connection.getAccountInfo(BET_KEY.publicKey);
  if (accountInfo === null) {
    throw 'Error: cannot find the greeted account';
  }

  console.log(
    BET_KEY.publicKey.toBase58(),
    'has been greeted',
  );

}


main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))
