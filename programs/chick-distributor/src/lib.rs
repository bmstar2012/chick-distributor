//! Program for distributing tokens efficiently via uploading a Merkle root.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    sysvar::{self},
    system_instruction::self,
};
use std::io::Write;

pub mod merkle_proof;

declare_id!("3NHg4RvfmsqtUL9N8kryZNfdCcYqsS92BQQnmiQdEJpn");

// fn get_or_create_claim_count<'a>(
//     distributor     : &Account<'a, MerkleDistributor>,
//     claim_count     : &AccountInfo<'a>,
//     temporal        : &Signer<'a>,
//     payer           : &Signer<'a>,
//     system_program  : &Program<'a, System>,
//     _claim_bump     : u8,
//     index           : u64,
//     claimant_secret : Pubkey,
// ) -> core::result::Result<anchor_lang::Account<'a, ClaimCount>, ProgramError> {
//     let rent = &Rent::get()?;
//     let space = 8 + ClaimCount::default().try_to_vec().unwrap().len();
//     let create_claim_state = claim_count.lamports() == 0; // TODO: support initial lamports?
//     if create_claim_state {
//         let lamports = rent.minimum_balance(space);
//         let claim_count_seeds = [
//             b"ClaimCount".as_ref(),
//             &index.to_le_bytes(),
//             &distributor.key().to_bytes(),
//             &[_claim_bump],
//         ];
//
//         invoke_signed(
//             &system_instruction::create_account(
//                 &payer.key(),
//                 claim_count.key,
//                 lamports,
//                 space as u64,
//                 &ID),
//             &[
//                 payer.to_account_info().clone(),
//                 claim_count.clone(),
//                 system_program.to_account_info().clone(),
//             ],
//             &[&claim_count_seeds],
//         )?;
//
//         let mut data = claim_count.try_borrow_mut_data()?;
//         let dst: &mut [u8] = &mut data;
//         let mut cursor = std::io::Cursor::new(dst);
//         cursor.write_all(&<ClaimCount as anchor_lang::Discriminator>::discriminator()).unwrap();
//     }
//
//     // anchor_lang::Account::try_from(&claim_count)?;
//     let mut pa: anchor_lang::Account<ClaimCount> =
//         anchor_lang::Account::try_from(&claim_count)?;
//
//     if create_claim_state {
//         require!(
//             temporal.key() == distributor.temporal
//             || temporal.key() == claimant_secret
//             || distributor.temporal == Pubkey::default()
//             ,
//             TemporalMismatch
//         );
//         pa.claimant = payer.key();
//     } else {
//         require!(
//             pa.claimant == payer.key(),
//             OwnerMismatch,
//         );
//     }
//
//     Ok(pa)
// }

/// The [merkle_distributor] program.
#[program]
pub mod merkle_distributor {
    use super::*;

    /// Creates a new [MerkleDistributor].
    /// After creating this [MerkleDistributor], the account should be seeded with tokens via
    /// delegates
    pub fn new_distributor(
        ctx: Context<NewDistributor>,
        bump: u8,
        root: [u8; 32],
        temporal: Pubkey,
    ) -> ProgramResult {
        let distributor = &mut ctx.accounts.distributor;

        msg!("new_distributor");

        distributor.base = ctx.accounts.base.key();
        distributor.bump = bump;

        distributor.root = root;
        distributor.temporal = temporal;

        Ok(())
    }

    /// Closes distributor-owned token accounts. Normal tokens should just use a delegate but we
    /// need to transfer ownership for edition minting ATM.
    // pub fn close_distributor_token_account(
    //     ctx: Context<CloseDistributorTokenAccount>,
    //     _bump: u8,
    // ) -> ProgramResult {
    //     let distributor = &ctx.accounts.distributor;
    //
    //     // should be implicit in the PDA
    //     require!(distributor.base == ctx.accounts.base.key(), Unauthorized);
    //
    //     let seeds = [
    //         b"MerkleDistributor".as_ref(),
    //         &distributor.base.to_bytes(),
    //         &[ctx.accounts.distributor.bump],
    //     ];
    //
    //     token::transfer(
    //         CpiContext::new(
    //             ctx.accounts.token_program.to_account_info(),
    //             token::Transfer {
    //                 from: ctx.accounts.from.to_account_info(),
    //                 to: ctx.accounts.to.to_account_info(),
    //                 authority: ctx.accounts.distributor.to_account_info(),
    //             },
    //         )
    //             .with_signer(&[&seeds[..]]),
    //         ctx.accounts.from.amount,
    //     )?;
    //
    //     token::close_account(
    //         CpiContext::new(
    //             ctx.accounts.token_program.to_account_info(),
    //             token::CloseAccount {
    //                 account: ctx.accounts.from.to_account_info(),
    //                 destination: ctx.accounts.receiver.to_account_info(),
    //                 authority: ctx.accounts.distributor.to_account_info(),
    //             },
    //         )
    //             .with_signer(&[&seeds[..]]),
    //     )?;
    //
    //     Ok(())
    // }

    // /// Closes an existing [MerkleDistributor].
    // /// Moves all tokens from the [MerkleDistributor] to the specified account and closes
    // /// distributor accounts.
    // /// Must `close_distributor_token_account` first
    // pub fn close_distributor<'info>(
    //     ctx: Context<'_, '_, '_, 'info, CloseDistributor<'info>>,
    //     _bump: u8,
    //     _wallet_bump: u8,
    // ) -> ProgramResult {
    //     let distributor = &ctx.accounts.distributor;
    //
    //     // should be implicit in the PDA
    //     require!(distributor.base == ctx.accounts.base.key(), Unauthorized);
    //
    //     let wallet_seeds = [
    //         b"Wallet".as_ref(),
    //         &distributor.key().to_bytes(),
    //         &[_wallet_bump],
    //     ];
    //
    //     if !ctx.remaining_accounts.is_empty() {
    //         // transfer authority out
    //         let candy_machine_info = &ctx.remaining_accounts[0];
    //         let candy_machine_program_info = &ctx.remaining_accounts[1];
    //
    //         // TODO. global::update_authority instruction...
    //         let mut data = vec![
    //             0x20, 0x2e, 0x40, 0x1c, 0x95, 0x4b, 0xf3, 0x58,
    //         ];
    //
    //         data.push(0x01);
    //         data.extend_from_slice(&ctx.accounts.receiver.key.to_bytes());
    //
    //         invoke_signed(
    //             &Instruction {
    //                 program_id: *candy_machine_program_info.key,
    //                 accounts: vec![
    //                     AccountMeta::new(*candy_machine_info.key, false),
    //                     AccountMeta::new(*ctx.accounts.distributor_wallet.key, true),
    //                 ],
    //                 data: data,
    //             },
    //             &[
    //                 candy_machine_info.clone(),
    //                 ctx.accounts.distributor_wallet.clone(),
    //             ],
    //             &[&wallet_seeds],
    //         )?;
    //     }
    //
    //     invoke_signed(
    //         &system_instruction::transfer(
    //             ctx.accounts.distributor_wallet.key,
    //             ctx.accounts.receiver.key,
    //             ctx.accounts.distributor_wallet.lamports(),
    //         ),
    //         &[
    //             ctx.accounts.distributor_wallet.clone(),
    //             ctx.accounts.receiver.clone(),
    //             ctx.accounts.system_program.to_account_info().clone(),
    //         ],
    //         &[&wallet_seeds],
    //     )?;
    //
    //     Ok(())
    // }

    /// Claims tokens from the [MerkleDistributor].
    pub fn claim(
        ctx: Context<Claim>,
        _bump: u8,
        index: u64,
        amount: u64,
        claimant_secret: Pubkey,
        proof: Vec<[u8; 32]>,
    ) -> ProgramResult {
        let claim_status = &mut ctx.accounts.claim_status;
        require!(
            *claim_status.to_account_info().owner == ID,
            OwnerMismatch
        );
        require!(
            // This check is redudant, we should not be able to initialize a claim status account at the same key.
            !claim_status.is_claimed && claim_status.claimed_at == 0,
            DropAlreadyClaimed
        );

        let distributor = &ctx.accounts.distributor;
        let mint = ctx.accounts.from.mint;

        // Verify the merkle proof.
        let node = solana_program::keccak::hashv(&[
            &[0x00],
            &index.to_le_bytes(),
            &claimant_secret.to_bytes(),
            &mint.to_bytes(),
            &amount.to_le_bytes(),
        ]);
        require!(
            merkle_proof::verify(proof, distributor.root, node.0),
            InvalidProof
        );

        // Mark it claimed and send the tokens.
        claim_status.amount = amount;
        claim_status.is_claimed = true;
        let clock = Clock::get()?;
        claim_status.claimed_at = clock.unix_timestamp;
        claim_status.claimant = ctx.accounts.payer.key();

        let seeds = [
            b"MerkleDistributor".as_ref(),
            &distributor.base.to_bytes(),
            &[ctx.accounts.distributor.bump],
        ];

        require!(
            ctx.accounts.temporal.key() == distributor.temporal
            || ctx.accounts.temporal.key() == claimant_secret
            || distributor.temporal == Pubkey::default()
            ,
            TemporalMismatch
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.distributor.to_account_info(),
                },
            )
                .with_signer(&[&seeds[..]]),
            amount,
        )?;

        emit!(ClaimedEvent {
            index,
            claimant: ctx.accounts.payer.key(),
            amount
        });
        Ok(())
    }

}

/// Accounts for [merkle_distributor::new_distributor].
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct NewDistributor<'info> {
    /// Base key of the distributor.
    pub base: Signer<'info>,

    /// [MerkleDistributor].
    #[account(
    init,
    seeds = [
    b"MerkleDistributor".as_ref(),
    base.key().to_bytes().as_ref()
    ],
    bump = bump,
    payer = payer
    )]
    pub distributor: Account<'info, MerkleDistributor>,

    /// Payer to create the distributor.
    pub payer: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,
}

/// [merkle_distributor::close_distributor_token_acconut] accounts.
#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct CloseDistributorTokenAccount<'info> {
    /// Base key of the distributor.
    pub base: Signer<'info>,

    /// [MerkleDistributor].
    #[account(
    seeds = [
    b"MerkleDistributor".as_ref(),
    base.key().to_bytes().as_ref()
    ],
    bump = _bump,
    )]
    pub distributor: Account<'info, MerkleDistributor>,

    /// Distributor containing the tokens to distribute.
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    /// Account to send the claimed tokens to.
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    /// Who is receiving the remaining rent allocation.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,

    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

/// [merkle_distributor::close_distributor] accounts.
#[derive(Accounts)]
#[instruction(_bump: u8, _wallet_bump: u8)]
pub struct CloseDistributor<'info> {
    /// Base key of the distributor.
    pub base: Signer<'info>,

    /// [MerkleDistributor].
    #[account(
    seeds = [
    b"MerkleDistributor".as_ref(),
    base.key().to_bytes().as_ref()
    ],
    bump = _bump,
    mut,
    close = receiver,
    )]
    pub distributor: Account<'info, MerkleDistributor>,

    #[account(
    seeds = [
    b"Wallet".as_ref(),
    distributor.key().to_bytes().as_ref()
    ],
    bump = _wallet_bump,
    mut,
    )]
    pub distributor_wallet: AccountInfo<'info>,

    /// Who is receiving the remaining tokens and rent allocations.
    pub receiver: AccountInfo<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,

    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

/// [merkle_distributor::claim] accounts.
#[derive(Accounts)]
#[instruction(_bump: u8, index: u64)]
pub struct Claim<'info> {
    /// The [MerkleDistributor].
    #[account(mut)]
    pub distributor: Account<'info, MerkleDistributor>,

    /// Status of the claim.
    #[account(
    init,
    seeds = [
    b"ClaimStatus".as_ref(),
    index.to_le_bytes().as_ref(),
    distributor.key().to_bytes().as_ref()
    ],
    bump = _bump,
    payer = payer
    )]
    pub claim_status: Account<'info, ClaimStatus>,

    /// Distributor containing the tokens to distribute.
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    /// Account to send the claimed tokens to.
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    /// Extra signer expected for claims
    pub temporal: Signer<'info>,

    /// Payer of the claim.
    pub payer: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,

    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

/// State for the account which distributes tokens.
#[account]
#[derive(Default)]
pub struct MerkleDistributor {
    /// Base key used to generate the PDA.
    pub base: Pubkey,
    /// Bump seed.
    pub bump: u8,

    /// The 256-bit merkle root.
    pub root: [u8; 32],

    /// Third-party signer expected on claims. Verified by OTP with off-chain distribution method
    pub temporal: Pubkey,
}

#[account]
#[derive(Default)]
pub struct ClaimStatus {
    /// If true, the tokens have been claimed.
    pub is_claimed: bool,
    /// Authority that claimed the tokens.
    pub claimant: Pubkey,
    /// When the tokens were claimed.
    pub claimed_at: i64,
    /// Amount of tokens claimed.
    pub amount: u64,
}

#[account]
#[derive(Default)]
pub struct ClaimCount {
    /// Number of NFTs claimed. Compared versus `amount` in merkle tree data / proof
    pub count: u64,
    /// Authority that claimed the tokens.
    pub claimant: Pubkey,
}

/// Emitted when tokens are claimed.
#[event]
pub struct ClaimedEvent {
    /// Index of the claim.
    pub index: u64,
    /// User that claimed.
    pub claimant: Pubkey,
    /// Amount of tokens to distribute.
    pub amount: u64,
}

// TODO: from cargo package...
#[account]
#[derive(Default)]
pub struct CandyMachine {
    pub authority: Pubkey,
    pub wallet: Pubkey,
    pub token_mint: Option<Pubkey>,
    pub config: Pubkey,
    pub data: CandyMachineData,
    pub items_redeemed: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CandyMachineData {
    pub uuid: String,
    pub price: u64,
    pub items_available: u64,
    pub go_live_date: Option<i64>,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid Merkle proof.")]
    InvalidProof,
    #[msg("Drop already claimed.")]
    DropAlreadyClaimed,
    #[msg("Account is not authorized to execute this instruction")]
    Unauthorized,
    #[msg("Token account owner did not match intended owner")]
    OwnerMismatch,
    #[msg("Temporal signer did not match distributor")]
    TemporalMismatch,
}
