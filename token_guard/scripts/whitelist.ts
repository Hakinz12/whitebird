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

const WALLETS_TO_WHITELIST = [
  "6GjrZoLnzbBDXnFBeAiuHrE7JYcmJJf9NV8jxZAkXad8",
  "2NdxatnEGRsPmZLVv8YTJGh1t6z4a4i33amVHiHGTQCD",
];

const RPC_URL = process.env.ANCHOR_PROVIDER_URL || "https://api.devnet.solana.com";
const WALLET_PATH = process.env.ANCHOR_WALLET || "/home/codespace/.config/solana/id.json";

function discriminator(name: string): Buffer {
  return crypto.createHash("sha256").update(`global:${name}`).digest().slice(0, 8);
}

async function main() {
  console.log(">>> Whitelist script started <<<");
  const connection = new Connection(RPC_URL, "confirmed");
  const admin = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(WALLET_PATH, "utf8"))));
  console.log("Admin wallet:", admin.publicKey.toBase58());

  const [lockConfigPda] = PublicKey.findProgramAddressSync([Buffer.from("lock_config"), MINT_ADDRESS.toBuffer()], PROGRAM_ID);

  for (const walletStr of WALLETS_TO_WHITELIST) {
    const wallet = new PublicKey(walletStr);
    const [whitelistPda] = PublicKey.findProgramAddressSync([Buffer.from("whitelist"), MINT_ADDRESS.toBuffer(), wallet.toBuffer()], PROGRAM_ID);
    console.log("\nWhitelisting: " + wallet.toBase58());
    console.log("  WhitelistEntry PDA: " + whitelistPda.toBase58());
    const ix = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        { pubkey: admin.publicKey, isSigner: true, isWritable: true },
        { pubkey: MINT_ADDRESS, isSigner: false, isWritable: false },
        { pubkey: lockConfigPda, isSigner: false, isWritable: false },
        { pubkey: whitelistPda, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: Buffer.concat([discriminator("add_to_whitelist"), wallet.toBuffer()]),
    });
    try {
      const sig = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [admin]);
      console.log("  Whitelisted! Tx: " + sig);
    } catch (e) {
      console.error("  Failed: " + (e.message || e));
    }
  }
  console.log("\n>>> Done <<<");
}

main().then(() => process.exit(0)).catch((e) => { console.error("Fatal error:", e); process.exit(1); });
