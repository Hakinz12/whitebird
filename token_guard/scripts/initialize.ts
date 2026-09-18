// @ts-nocheck
declare const Buffer: any;
declare const process: any;

import {
  Connection, Keypair, PublicKey, SystemProgram,
  Transaction, TransactionInstruction, sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as fs from "fs";
import * as crypto from "crypto";
import * as dotenv from "dotenv";
dotenv.config();

const PROGRAM_ID = new PublicKey("771om81G5Ey5z1M6cb5NEu6s7dZmapUY3zwBgR4qU6Z9");
const MINT_ADDRESS = new PublicKey("7LfBjByaje55A1GnTmUEkVin38WY9H3RKay2K4AKh6T9");
const UNLOCK_TIMESTAMP = 1789718400;

const RPC_URL = process.env.ANCHOR_PROVIDER_URL || "https://api.devnet.solana.com";
const WALLET_PATH = process.env.ANCHOR_WALLET || "/home/codespace/.config/solana/id.json";

function discriminator(name: string): Buffer {
  return crypto.createHash("sha256").update(`global:${name}`).digest().slice(0, 8);
}

function encodeI64(value: number): Buffer {
  const buf = Buffer.alloc(8);
  buf.writeBigInt64LE(BigInt(value), 0);
  return buf;
}

async function main() {
  console.log(">>> Script started <<<");
  const connection = new Connection(RPC_URL, "confirmed");
  const admin = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(WALLET_PATH, "utf8"))));
  console.log("Admin wallet:", admin.publicKey.toBase58());
  console.log("Mint address:", MINT_ADDRESS.toBase58());

  const [lockConfigPda] = PublicKey.findProgramAddressSync([Buffer.from("lock_config"), MINT_ADDRESS.toBuffer()], PROGRAM_ID);
  const [extraMetaPda] = PublicKey.findProgramAddressSync([Buffer.from("extra-account-metas"), MINT_ADDRESS.toBuffer()], PROGRAM_ID);
  console.log("LockConfig PDA:", lockConfigPda.toBase58());
  console.log("ExtraAccountMetaList PDA:", extraMetaPda.toBase58());

  const initLockIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: admin.publicKey, isSigner: true, isWritable: true },
      { pubkey: MINT_ADDRESS, isSigner: false, isWritable: false },
      { pubkey: lockConfigPda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: Buffer.concat([discriminator("initialize_lock_config"), encodeI64(UNLOCK_TIMESTAMP)]),
  });
  const sig1 = await sendAndConfirmTransaction(connection, new Transaction().add(initLockIx), [admin]);
  console.log("LockConfig initialized! Tx:", sig1);

  const initMetaIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: admin.publicKey, isSigner: true, isWritable: true },
      { pubkey: MINT_ADDRESS, isSigner: false, isWritable: false },
      { pubkey: extraMetaPda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: discriminator("initialize_extra_account_meta_list"),
  });
  const sig2 = await sendAndConfirmTransaction(connection, new Transaction().add(initMetaIx), [admin]);
  console.log("ExtraAccountMetaList initialized! Tx:", sig2);
}

main().then(() => process.exit(0)).catch((e) => { console.error("Fatal error:", e); process.exit(1); });
