// ============================================================================
// Superteam Indonesia — Weekend Class Lesson 6
// WORKSHOP FILE — Isi bagian TODO untuk menyelesaikan escrow program!
//
// Instruksi:
//  1. Baca setiap fungsi dan komentar dengan seksama
//  2. Isi bagian yang ditandai // TODO
//  3. Gunakan lib.rs sebagai referensi jika stuck
//  4. Jalankan `anchor test` untuk cek jawabanmu
//
// Tips:
//  - Setiap TODO memiliki petunjuk yang jelas
//  - Urutan pengerjaan: make() → take() → cancel()
//  - PDA signing adalah konsep paling baru — fokus di sini!
// ============================================================================

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ─── State ────────────────────────────────────────────────────────────────────
// Struct ini sudah diisi — perhatikan field-field yang disimpan on-chain.
// InitSpace memungkinkan Anchor menghitung ukuran account secara otomatis.

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub maker: Pubkey,  // Wallet yang membuat escrow
    pub mint_a: Pubkey, // Token yang ditawarkan maker
    pub mint_b: Pubkey, // Token yang diminta maker dari taker
    pub receive: u64,   // Jumlah mint_b yang diminta
    pub bump: u8,       // Bump untuk PDA signing
}

// ─── Custom Errors ────────────────────────────────────────────────────────────

#[error_code]
pub enum EscrowError {
    #[msg("Vault is empty — nothing to transfer")]
    EmptyVault,
    #[msg("Invalid escrow state")]
    InvalidEscrow,
}

// ─── Events ───────────────────────────────────────────────────────────────────

#[event]
pub struct EscrowMade {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
}

#[event]
pub struct EscrowTaken {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub amount_a: u64,
}

#[event]
pub struct EscrowCancelled {
    pub maker: Pubkey,
}

// ─── Program ─────────────────────────────────────────────────────────────────

#[program]
pub mod escrow {
    use super::*;

    // =========================================================================
    // INSTRUKSI 1: make()
    //
    // Yang harus dilakukan:
    //  a. Simpan state ke ctx.accounts.escrow menggunakan set_inner()
    //  b. Transfer token_a dari maker ke vault via CPI ke Token Program
    //
    // Fungsi ini dipanggil oleh MAKER — maker menandatangani transaksi,
    // jadi transfer biasa (tanpa PDA signing) sudah cukup.
    // =========================================================================
    pub fn make(ctx: Context<Make>, receive: u64) -> Result<()> {
        // TODO 1a: Simpan state ke escrow account.
        //
        // Gunakan ctx.accounts.escrow.set_inner(Escrow { ... })
        // Field yang perlu diisi:
        //   - maker     → ambil dari ctx.accounts.maker.key()
        //   - mint_a    → ambil dari ctx.accounts.mint_a.key()
        //   - mint_b    → ambil dari ctx.accounts.mint_b.key()
        //   - receive   → parameter fungsi ini
        //   - bump      → ctx.bumps.escrow (Anchor sudah hitung ini)
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Isi state escrow dengan set_inner()");

        // TODO 1b: Transfer token_a dari maker ke vault.
        //
        // Langkah-langkah:
        //  1. Ambil amount dari ctx.accounts.maker_ata_a.amount
        //  2. Cek amount > 0 dengan require!(amount > 0, EscrowError::EmptyVault)
        //  3. Panggil transfer() dengan CpiContext::new(...)
        //
        // Transfer { from: maker_ata_a, to: vault, authority: maker }
        //
        // Ingat: maker adalah Signer sehingga bisa langsung jadi authority.
        // Gunakan .to_account_info() untuk setiap account.
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Transfer token_a dari maker_ata_a ke vault");

        emit!(EscrowMade {
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            receive,
        });

        Ok(())
    }

    // =========================================================================
    // INSTRUKSI 2: take()
    //
    // Yang harus dilakukan:
    //  a. Transfer token_b dari taker ke maker (taker menandatangani)
    //  b. Transfer token_a dari vault ke taker (PDA yang menandatangani!)
    //
    // KONSEP BARU — PDA Signing:
    //  Vault dimiliki oleh escrow PDA. PDA tidak punya private key,
    //  jadi kita harus signing dengan seeds menggunakan CpiContext::new_with_signer()
    //
    // Format seeds: &[b"escrow", maker_key.as_ref(), &[bump]]
    // =========================================================================
    pub fn take(ctx: Context<Take>) -> Result<()> {
        // TODO 2a: Transfer token_b dari taker ke maker.
        //
        // Ini transfer biasa — taker adalah Signer sehingga bisa jadi authority.
        // Jumlah yang ditransfer: ctx.accounts.escrow.receive
        //
        // Transfer { from: taker_ata_b, to: maker_ata_b, authority: taker }
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Transfer token_b dari taker ke maker");

        // TODO 2b: Bangun PDA seeds untuk signing.
        //
        // Seeds harus sama persis dengan yang digunakan saat membuat PDA di Make:
        //   seeds = [b"escrow", maker.key().as_ref()]
        //   plus bump yang disimpan di escrow state
        //
        // Cara membuat seeds slice:
        //   let maker_key = ctx.accounts.maker.key();
        //   let seeds: &[&[u8]] = &[b"escrow", maker_key.as_ref(), &[ctx.accounts.escrow.bump]];
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Bangun seeds untuk PDA signing");

        // TODO 2c: Transfer token_a dari vault ke taker menggunakan PDA signing.
        //
        //  1. Ambil amount dari ctx.accounts.vault.amount
        //  2. Cek amount > 0 dengan require!()
        //  3. Gunakan CpiContext::new_with_signer(...) bukan CpiContext::new()
        //  4. Signer seeds format: &[seeds] (satu set seeds, bukan nested)
        //
        // Transfer { from: vault, to: taker_ata_a, authority: escrow }
        // (escrow PDA adalah authority vault karena vault dibuat dengan authority = escrow)
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Transfer token_a dari vault ke taker dengan PDA signing");

        emit!(EscrowTaken {
            taker: ctx.accounts.taker.key(),
            maker: ctx.accounts.maker.key(),
            amount_a: 0, // TODO: ganti 0 dengan amount yang kamu ambil dari vault
        });

        Ok(())
    }

    // =========================================================================
    // INSTRUKSI 3: cancel()
    //
    // Yang harus dilakukan:
    //  a. Bangun PDA seeds (sama seperti take())
    //  b. Transfer token_a dari vault kembali ke maker_ata_a dengan PDA signing
    //
    // Mirip dengan take(), tapi lebih sederhana karena hanya satu transfer.
    // Tidak ada token_b yang terlibat.
    // =========================================================================
    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        // TODO 3a: Bangun PDA seeds.
        //
        // Sama persis dengan TODO 2b di atas.
        // Seeds: [b"escrow", maker_key.as_ref(), bump]
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Bangun seeds untuk PDA signing");

        // TODO 3b: Transfer token_a dari vault ke maker menggunakan PDA signing.
        //
        //  1. Ambil amount dari ctx.accounts.vault.amount
        //  2. Cek amount > 0
        //  3. CpiContext::new_with_signer()
        //
        // Transfer { from: vault, to: maker_ata_a, authority: escrow }
        //
        // Hapus baris di bawah dan isi implementasimu:
        todo!("Transfer token_a dari vault ke maker dengan PDA signing");

        emit!(EscrowCancelled {
            maker: ctx.accounts.maker.key(),
        });

        Ok(())
    }
}

// ─── Account Contexts ─────────────────────────────────────────────────────────
// Account contexts sudah diisi lengkap — constraints Anchor cukup kompleks
// sehingga kita berikan sebagai referensi. Fokus pada implementasi instruksi di atas.

/// Akun-akun yang dibutuhkan instruksi Make
#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    // ATA maker untuk mint_a — sumber token yang akan masuk vault
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    // Escrow PDA — dibuat oleh instruksi ini, seed: [b"escrow", maker.key]
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // Vault — ATA yang dikontrol escrow PDA (bukan maker!)
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Akun-akun yang dibutuhkan instruksi Take
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    // Maker hanya sebagai penerima — has_one di escrow memverifikasi address ini
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    // ATA taker untuk mint_b (dikirim ke maker)
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    // ATA taker untuk mint_a (diterima dari vault)
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,

    // ATA maker untuk mint_b (penerima token_b)
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,

    // Escrow PDA — has_one memverifikasi maker, mint_a, mint_b sesuai
    // close = maker: setelah instruksi selesai, account ditutup & rent dikembalikan
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref()],
        bump = escrow.bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    // Vault — dikosongkan lalu ditutup
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        close = maker,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Akun-akun yang dibutuhkan instruksi Cancel
#[derive(Accounts)]
pub struct Cancel<'info> {
    // Maker adalah Signer — hanya maker yang bisa cancel escrow miliknya
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,

    // ATA maker untuk mint_a — penerima token yang dikembalikan dari vault
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    // has_one = maker memastikan hanya maker yang bisa cancel
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref()],
        bump = escrow.bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    // Vault — isi dikembalikan ke maker, lalu ditutup
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        close = maker,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
