/**
 * Solana Token Workshop - Lesson 2 — File Latihan Peserta
 *
 * Instruksi:
 * - Isi setiap blok // TODO: sesuai penjelasan instruktur
 * - File jawaban lengkap ada di: solana-token-transfer.ts
 * - Jalankan dengan: npx ts-node solana-token-transfer.workshop.ts
 */

import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';

import * as fs from "fs";

import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  burn,
  setAuthority,
  AuthorityType,
  getAccount,
} from '@solana/spl-token';
import { fstat } from 'fs';

// ============================================================================
// KONFIGURASI
// ============================================================================

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

const TOKEN_DECIMALS = 9;   // Presisi token: 9 desimal seperti SOL
const MINT_AMOUNT = 1000;   // Jumlah token yang dicetak
const TRANSFER_AMOUNT = 50; // Jumlah token yang ditransfer ke Wallet B
const BURN_AMOUNT = 100;    // Jumlah token yang dibakar dari Wallet A

// ============================================================================
// FUNGSI HELPER
// ============================================================================

/**
 * Cek dan tampilkan saldo SOL sebuah wallet.
 * Saldo di Solana disimpan dalam "lamports" (satuan terkecil).
 * 1 SOL = 1.000.000.000 lamports
 */
async function checkSolBalance(publicKey: PublicKey, label: string): Promise<void> {
  // TODO: Ambil saldo dengan connection.getBalance(publicKey)
  // TODO: Bagi dengan LAMPORTS_PER_SOL untuk konversi ke SOL
  // TODO: Tampilkan hasilnya dengan console.log
    const balance = await connection.getBalance(publicKey);
    const sol = balance / LAMPORTS_PER_SOL;
    console.log(`  ${label}: ${sol.toFixed(4)} SOL`);
}

/**
 * Cek dan tampilkan saldo token dari sebuah Associated Token Account.
 * Amount disimpan dalam unit terkecil, bagi dengan 10^TOKEN_DECIMALS
 * untuk mendapatkan jumlah token yang sebenarnya.
 */
async function checkTokenBalance(tokenAccount: PublicKey): Promise<void> {
  // TODO: Ambil info account dengan getAccount(connection, tokenAccount)
  // TODO: Bagi accountInfo.amount dengan Math.pow(10, TOKEN_DECIMALS)
  // TODO: Tampilkan hasilnya dengan console.log
    const accountInfo = await getAccount(connection, tokenAccount);
    const balance = Number(accountInfo.amount) / Math.pow(10, TOKEN_DECIMALS);
    console.log(`  ${tokenAccount.toBase58().slice(0, 8)}...: ${balance} token`);
}

// ============================================================================
// PROGRAM UTAMA
// ============================================================================

async function main() {
  console.log('\n╔════════════════════════════════════════════════════════╗');
  console.log('║          SOLANA TOKEN WORKSHOP - Lesson 2             ║');
  console.log('╚════════════════════════════════════════════════════════╝');

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 1 — Buat Wallet
  // Keypair = pasangan public key + private key
  // Public key  = alamat wallet (aman dibagikan)
  // Private key = kunci rahasia untuk tanda tangan transaksi
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 1: Buat Wallet ---');
  // TODO: Buat walletA dengan Keypair.generate()
  const secretKeyA = Uint8Array.from(
    JSON.parse(fs.readFileSync("walletA-privateKey.txt", "utf-8")),
  );
  const walletA = Keypair.fromSecretKey(secretKeyA);
  // TODO: Buat walletB dengan Keypair.generate()
  const secretKeyB = Uint8Array.from(
    JSON.parse(fs.readFileSync("walletB-privateKey.txt", "utf-8")),
  );
  const walletB = Keypair.fromSecretKey(secretKeyB);
  // TODO: Tampilkan public key masing-masing wallet
  console.log('Wallet A:', walletA.publicKey.toBase58());
  console.log('Wallet B:', walletB.publicKey.toBase58());

  // TODO: Kode untuk menangkap wallet yang sudah kita gunakan tadi dalam bentuk txt
  // fs.writeFile(
  //   "walletA-privateKey.txt",
  //   walletA.secretKey.toString(),
  //   "utf8",
  //   (err) => {
  //     if (err) {
  //       console.error("Error writing file", err);
  //     } else {
  //       console.log("Successfully wrote file");
  //     }
  //   },
  // );

  // fs.writeFile(
  //   "walletB-privateKey.txt",
  //   walletB.secretKey.toString(),
  //   "utf8",
  //   (err) => {
  //     if (err) {
  //       console.error("Error writing file", err);
  //     } else {
  //       console.log("Successfully wrote file");
  //     }
  //   },
  // );
  
  // BUAT REQUEST AIRDROP
  // TODO: Minta airdrop 2 SOL untuk walletA
  //   const sig = await connection.requestAirdrop(walletA.publicKey, 2 * LAMPORTS_PER_SOL)
  //   await connection.confirmTransaction(sig)
  // TODO: Minta airdrop 1 SOL untuk walletB (pola yang sama)
  // const sigA = await connection.requestAirdrop(
  //   walletA.publicKey, 1 * LAMPORTS_PER_SOL)
  // console.log('\nSaldo SOL:');

  // const sigB = await connection.requestAirdrop(
  //   walletB.publicKey, 1 * LAMPORTS_PER_SOL)
  // console.log('\nSaldo SOL:');
  // TODO: Panggil checkSolBalance() untuk walletA dan walletB
  console.log('\nSaldo SOL: ')
  await checkSolBalance(walletA.publicKey, 'Wallet A');
  await checkSolBalance(walletB.publicKey, 'Wallet B');

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 2 — Buat Token Mint
  // Token Mint = "pabrik" token. Menyimpan: mint authority, freeze authority, decimals.
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 2: Buat Token Mint ---');

  // TODO: Buat token mint dengan createMint()
  //   const mint = await createMint(connection, payer, mintAuthority, freezeAuthority, decimals)
  //   Gunakan walletA sebagai payer, mint authority, dan freeze authority
  const payer = walletA;
  const mintAuthority = walletA.publicKey;
  const freezeAuthority = walletA.publicKey;
  const decimals = TOKEN_DECIMALS;
  
  const mint = await createMint(
    connection,
    payer,
    walletA.publicKey,
    walletA.publicKey,
    TOKEN_DECIMALS
  );

  // TODO: Tampilkan mint.toBase58() dan link explorer devnet
  console.log ("mint address: ", mint.toBase58())

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 3 — Mint Token ke Wallet A
  // Token di Solana tidak tersimpan di wallet langsung.
  // Setiap wallet butuh Associated Token Account (ATA) per jenis token.
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 3: Mint Token ke Wallet A ---');

  // TODO: Buat ATA untuk walletA dengan getOrCreateAssociatedTokenAccount()
  const tokenAccountA = await getOrCreateAssociatedTokenAccount(
       connection, walletA, mint, walletA.publicKey
     )
  // TODO: Tampilkan tokenAccountA.address.toBase58()
     console.log ("token account a: ", tokenAccountA.address.toBase58());
  // TODO: Cetak token ke tokenAccountA dengan mintTo()
  //   Ingat: jumlah = MINT_AMOUNT * Math.pow(10, TOKEN_DECIMALS)
     await mintTo (
      connection,
      walletA,
      mint,
      tokenAccountA.address,
      walletA,
      MINT_AMOUNT * Math.pow(10, TOKEN_DECIMALS),
     );

  console.log(`${MINT_AMOUNT} token berhasil dicetak!`);
  console.log('Saldo token:');
  // TODO: Panggil checkTokenBalance(tokenAccountA.address)

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 4 — Transfer Token ke Wallet B
  // walletB butuh ATA-nya sendiri sebelum bisa menerima token.
  // walletA yang membayar biaya pembuatan ATA walletB.
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 4: Transfer Token ke Wallet B ---');

  // TODO: Buat ATA untuk walletB dengan getOrCreateAssociatedTokenAccount()
     const tokenAccountB = await getOrCreateAssociatedTokenAccount(
       connection, 
       walletA, 
       mint, 
       walletB.publicKey
     )
  // TODO: Tampilkan tokenAccountB.address.toBase58()
    console.log ("token account b: ", tokenAccountB.address.toBase58());

  // TODO: Transfer token dengan transfer()
  //   Ingat: jumlah = TRANSFER_AMOUNT * Math.pow(10, TOKEN_DECIMALS)
     await transfer (
      connection,
      walletA,
      tokenAccountA.address,
      tokenAccountB.address,
      walletA,
      TRANSFER_AMOUNT * Math.pow(10, TOKEN_DECIMALS),
     );

  console.log(`${TRANSFER_AMOUNT} token berhasil ditransfer!`);
  console.log('Saldo token setelah transfer:');
  await checkTokenBalance(tokenAccountA.address);
  await checkTokenBalance(tokenAccountB.address);
  // TODO: Panggil checkTokenBalance() untuk tokenAccountA dan tokenAccountB

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 5 — Burn Token dari Wallet A
  // Burn = menghancurkan token secara permanen. Supply berkurang.
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 5: Burn Token dari Wallet A ---');

  // TODO: Bakar token dengan burn()
  //   Ingat: jumlah = BURN_AMOUNT * Math.pow(10, TOKEN_DECIMALS)

  console.log(`${BURN_AMOUNT} token berhasil dibakar!`);
  console.log('Saldo token setelah burn:');
  // TODO: Panggil checkTokenBalance(tokenAccountA.address)

  // ─────────────────────────────────────────────────────────────────────────
  // Langkah 6 — Nonaktifkan Minting (Revoke Mint Authority)
  // Serahkan mint authority ke `null` = tidak ada yang bisa cetak token lagi.
  // Supply menjadi TETAP selamanya. Tindakan ini TIDAK BISA DIBALIK!
  // ─────────────────────────────────────────────────────────────────────────
  console.log('\n--- Langkah 6: Nonaktifkan Minting ---');

  // TODO: Cabut mint authority dengan setAuthority()
  //   await setAuthority(connection, walletA, mint, walletA, AuthorityType.MintTokens, null)
  //   Parameter terakhir `null` = tidak ada pemilik baru = permanen dinonaktifkan

  console.log('Mint authority berhasil dicabut!');
  console.log('Supply token sekarang TETAP selamanya — tidak bisa dicetak lagi.');

  console.log('\nWorkshop selesai!');
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });