use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgQQTV5Lcb8a");

#[program]
pub mod open_books {
    use super::*;

    // Initialize function to set up the book rental with an owner and rental price per day
    pub fn initialize(ctx: Context<Initialize>, rent_price_per_day: u64) -> ProgramResult {
        let book_rental = &mut ctx.accounts.book_rental;
        book_rental.owner = *ctx.accounts.owner.key;
        book_rental.rent_price_per_day = rent_price_per_day;
        Ok(())
    }

    // Function to rent a book for a specified number of days
    pub fn rent_book(ctx: Context<RentBook>, days: u64) -> ProgramResult {
        let book_rental = &mut ctx.accounts.book_rental;
        let rent_amount = book_rental.rent_price_per_day.checked_mul(days).ok_or(ErrorCode::Overflow)?;
        
        // Transfer rent amount from renter to the book rental account
        **book_rental.to_account_info().try_borrow_mut_lamports()? += rent_amount;
        **ctx.accounts.renter.to_account_info().try_borrow_mut_lamports()? -= rent_amount;

        // Update rental details in the book rental account
        book_rental.renter = Some(*ctx.accounts.renter.key);
        book_rental.rental_duration = Some(days);
        book_rental.rental_start_time = Some(Clock::get()?.unix_timestamp);

        Ok(())
    }

    // Function to return the rented book after the rental period is over
    pub fn return_book(ctx: Context<ReturnBook>) -> ProgramResult {
        let book_rental = &mut ctx.accounts.book_rental;

        // Ensure the book was rented
        let rental_start_time = book_rental.rental_start_time.ok_or(ErrorCode::NotRented)?;
        let rental_duration = book_rental.rental_duration.ok_or(ErrorCode::NotRented)?;

        // Check if the rental period is over
        let current_time = Clock::get()?.unix_timestamp;
        if current_time >= rental_start_time + (rental_duration as i64 * 86400) {
            let rent_amount = book_rental.rent_price_per_day.checked_mul(rental_duration).ok_or(ErrorCode::Overflow)?;

            // Transfer rent amount from book rental account to owner
            **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += rent_amount;
            **book_rental.to_account_info().try_borrow_mut_lamports()? -= rent_amount;

            // Clear rental details in the book rental account
            book_rental.renter = None;
            book_rental.rental_duration = None;
            book_rental.rental_start_time = None;
        } else {
            return Err(ErrorCode::RentalPeriodNotOver.into());
        }

        Ok(())
    }
}

// Account structure and initialization for book rental
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 40)]
    pub book_rental: Account<'info, BookRental>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Account structure for renting a book
#[derive(Accounts)]
pub struct RentBook<'info> {
    #[account(mut, has_one = owner)]
    pub book_rental: Account<'info, BookRental>,
    #[account(mut)]
    pub renter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Account structure for returning a rented book
#[derive(Accounts)]
pub struct ReturnBook<'info> {
    #[account(mut, has_one = owner, has_one = renter)]
    pub book_rental: Account<'info, BookRental>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub renter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Data structure to store book rental details
#[account]
pub struct BookRental {
    pub owner: Pubkey,
    pub renter: Option<Pubkey>,
    pub rent_price_per_day: u64,
    pub rental_duration: Option<u64>,
    pub rental_start_time: Option<i64>,
}

// Error codes for the smart contract
#[error]
pub enum ErrorCode {
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Book is not currently rented.")]
    NotRented,
    #[msg("Rental period is not over yet.")]
    RentalPeriodNotOver,
}
